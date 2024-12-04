pub use super::*;
pub use command::gaussian_3d::RenderArguments;
pub use gausplat_loader::source::image::Image;

use color_eyre::eyre::eyre;
use std::{
    fmt, fs,
    path::{Path, PathBuf},
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
    #[inline]
    fn get_image_from_tensor(tensor: Tensor<Wgpu, 3>) -> Image {
        let mut image = Image {
            image_file_path: " .png".into(),
            ..Default::default()
        };
        // NOTE: Assume that the tensor is in correct shape.
        image.encode_rgb_tensor(tensor).unwrap();
        image
    }

    #[inline]
    pub fn make_directory(directory: PathBuf) -> Result<PathBuf, Report> {
        fs::remove_dir_all(&directory).or_else(|_| fs::create_dir_all(&directory))?;
        fs::create_dir_all(&directory)?;
        Ok(directory)
    }

    #[inline]
    pub fn save_image(
        directory: impl AsRef<Path>,
        index: usize,
        mut image: Image,
    ) -> Result<(), Report> {
        image.image_file_path = directory.as_ref().join(format!("{index:05}.png"));
        image.save()?;
        Ok(())
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
        let test_size = self.cameras_test.len();
        let train_size = self.cameras_train.len();

        // Specifying the progress bar

        let mut bar = get_bar();
        bar.colour = Some("ansi(45)".into());
        bar.desc = "| Painting 3DGS".into();
        bar.disable = quiet != 0;
        bar.mininterval = 0.005;
        bar.postfix = format!(" Iteration {iteration} |");

        // Rendering and saving the images

        bar.total = test_size;
        self.cameras_test
            .into_values()
            .enumerate()
            .try_for_each(|(index, camera)| {
                Self::save_image(
                    &dir_test_rendered,
                    index,
                    Self::get_image_from_tensor(
                        self.scene
                            .render(&camera.view, &options_renderer)?
                            .colors_rgb_2d,
                    ),
                )?;
                Self::save_image(&dir_test_true, index, camera.image)?;
                bar.update(1)?;

                Ok::<_, Report>(())
            })?;
        if !bar.disable && test_size != 0 {
            eprintln!();
        }

        bar.total = train_size;
        self.cameras_train.into_values().enumerate().try_for_each(
            |(index, camera)| {
                Self::save_image(
                    &dir_train_rendered,
                    index,
                    Self::get_image_from_tensor(
                        self.scene
                            .render(&camera.view, &options_renderer)?
                            .colors_rgb_2d,
                    ),
                )?;
                Self::save_image(&dir_train_true, index, camera.image)?;
                bar.update(1)?;

                Ok::<_, Report>(())
            },
        )?;
        if !bar.disable && train_size != 0 {
            eprintln!();
        }

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
