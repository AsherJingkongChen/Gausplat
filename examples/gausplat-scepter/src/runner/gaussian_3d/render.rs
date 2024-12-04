pub use super::*;
pub use command::gaussian_3d::RenderArguments;
pub use gausplat_loader::source::image::Image;

use color_eyre::eyre::eyre;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use std::{
    convert::identity,
    fmt, fs,
    path::{Path, PathBuf},
    time::Instant,
};

#[derive(Clone)]
pub struct RenderRunner {
    pub arguments: RenderArguments,
    pub cameras_test: Cameras,
    pub cameras_train: Cameras,
    pub iteration: u64,
    pub scene: Gaussian3dScene<Wgpu>,
}

impl RenderArguments {
    pub fn init(&self) -> Result<RenderRunner, Report> {
        let arguments = self.to_owned();

        // Loading the cameras and points

        let (cameras_test, cameras_train, _) =
            get_cameras_and_points(&self.common_arguments)?;

        let model_directory = self.common_arguments.model_path.join("point_cloud");
        let iteration = match self.iteration {
            Some(iteration) => iteration,
            None => {
                // Finding the maximum iteration
                fs::read_dir(&model_directory)?
                    .filter_map(|entry| Some(entry.ok()?.path()))
                    .filter(|path| path.is_dir())
                    .filter_map(|path| {
                        path.file_name()?
                            .to_str()?
                            .split('_')
                            .next_back()?
                            .parse::<u64>()
                            .ok()
                    })
                    .max()
                    .ok_or_else(|| {
                        eyre!("Unrecognizable model directory: {model_directory:?}")
                    })?
            },
        };
        let model_file_path = [
            model_directory.as_path(),
            format!("iteration_{iteration}").as_ref(),
            "point_cloud.ply".as_ref(),
        ]
        .iter()
        .collect::<PathBuf>();

        // Loading the scene

        let device = WgpuDevice::default();
        let scene =
            Gaussian3dScene::decode_polygon(&mut File::open(model_file_path)?, &device)?;

        Ok(RenderRunner {
            arguments,
            cameras_test,
            cameras_train,
            iteration,
            scene,
        })
    }
}

impl RenderRunner {
    /// Obtaining the rendered and true images from the `cameras` and `scene`.
    pub fn get_images_rendered_and_true(
        bar: &mut Bar,
        cameras: Cameras,
        options: &Gaussian3dRenderOptions,
        scene: &Gaussian3dScene<Wgpu>,
    ) -> Result<(Vec<Image>, Vec<Image>), Report> {
        let size = cameras.len();
        let should_show_progress = !bar.disable && size != 0;

        bar.counter = 0;
        bar.total = size;
        if should_show_progress {
            bar.refresh()?;
        }
        let images_pair_result = cameras.into_values().try_fold(
            (Vec::with_capacity(size), Vec::with_capacity(size)),
            |mut images, camera| {
                let mut image_rendered = Image {
                    image_file_path: "_.png".into(),
                    ..Default::default()
                };
                image_rendered.encode_rgb_tensor(
                    scene.render(&camera.view, options)?.colors_rgb_2d,
                )?;
                images.0.push(image_rendered);
                images.1.push(camera.image);
                bar.update(1)?;

                Ok(images)
            },
        );
        if should_show_progress {
            eprintln!();
        }

        images_pair_result
    }

    /// Creating a new `directory`.
    #[inline]
    pub fn make_directory(directory: PathBuf) -> Result<PathBuf, Report> {
        fs::remove_dir_all(&directory).or_else(|_| fs::create_dir_all(&directory))?;
        fs::create_dir_all(&directory)?;
        Ok(directory)
    }

    /// Saving the `images` to the `directory`.
    #[inline]
    pub fn save_images(
        directory: impl AsRef<Path>,
        images: Vec<Image>,
    ) -> impl ParallelIterator<Item = Result<(), Report>> {
        let directory = directory.as_ref().to_owned();
        images
            .into_par_iter()
            .enumerate()
            .map(move |(index, mut image)| {
                image.image_file_path = directory.join(format!("{index:05}.png"));
                image.save()?;
                Ok(())
            })
    }
}

impl Runner for RenderRunner {
    fn run(mut self) -> Result<(), Report> {
        // Specifying the parameters

        let iteration = self.iteration;
        let model_path = self.arguments.common_arguments.model_path.as_path();
        let options_renderer = Gaussian3dRenderOptions::default()
            .with_colors_sh_degree_max(self.arguments.sh_degree);
        let quiet = self.arguments.common_arguments.quiet;
        let directory_test = [
            model_path,
            "test".as_ref(),
            format!("ours_{iteration}").as_ref(),
        ]
        .into_iter()
        .collect::<PathBuf>();
        let directory_train = [
            model_path,
            "train".as_ref(),
            format!("ours_{iteration}").as_ref(),
        ]
        .into_iter()
        .collect::<PathBuf>();

        // Specifying the directories

        let dir_test_rendered = Self::make_directory(directory_test.join("renders"))?;
        let dir_test_true = Self::make_directory(directory_test.join("gt"))?;
        let dir_train_rendered = Self::make_directory(directory_train.join("renders"))?;
        let dir_train_true = Self::make_directory(directory_train.join("gt"))?;

        // Skipping the specified target

        if self.arguments.skip_test {
            self.cameras_test.clear();
        }
        if self.arguments.skip_train {
            self.cameras_train.clear();
        }

        // Specifying the progress bar

        let mut bar = get_bar();
        bar.colour = Some("ansi(45)".into());
        bar.desc = "| Printing 3DGS".into();
        bar.disable = quiet != 0;
        bar.mininterval = 0.005;
        bar.postfix = format!(" Iteration {iteration} |");

        // Obtaining the rendered and true images

        let (imgs_test_rendered, imgs_test_true) = Self::get_images_rendered_and_true(
            &mut bar,
            self.cameras_test,
            &options_renderer,
            &self.scene,
        )?;
        let (imgs_train_rendered, imgs_train_true) = Self::get_images_rendered_and_true(
            &mut bar,
            self.cameras_train,
            &options_renderer,
            &self.scene,
        )?;

        // Saving the images

        let time = Instant::now();
        rayon::iter::empty()
            .chain(Self::save_images(&dir_test_rendered, imgs_test_rendered))
            .chain(Self::save_images(&dir_test_true, imgs_test_true))
            .chain(Self::save_images(&dir_train_rendered, imgs_train_rendered))
            .chain(Self::save_images(&dir_train_true, imgs_train_true))
            .try_for_each(identity)?;

        log::info!(
            target: "gausplat::scepter::gaussian_3d::render",
            "save in {:.03?}", time.elapsed(),
        );

        Ok(())
    }
}

impl fmt::Debug for RenderRunner {
    #[inline]
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.debug_struct("RenderRunner")
            .field("arguments", &self.arguments)
            .field("cameras_test.len()", &self.cameras_test.len())
            .field("cameras_train.len()", &self.cameras_train.len())
            .field("iteration", &self.iteration)
            .field("scene", &self.scene)
            .finish()
    }
}
