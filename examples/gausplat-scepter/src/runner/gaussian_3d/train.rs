pub use super::*;
pub use command::gaussian_3d::TrainArguments;
pub use gausplat_trainer::train::gaussian_3d::Gaussian3dTrainerConfig;

use gausplat_trainer::{
    metric::{Metric, Psnr},
    optimize::LearningRateConfig,
    range::RangeOptions,
    train::gaussian_3d::{
        Autodiff, AutodiffModule, Gaussian3dTrainer, RefinerConfig, SEED,
    },
};
use std::{
    fmt, fs,
    path::{Path, PathBuf},
};

#[derive(Clone)]
pub struct TrainRunner {
    pub arguments: TrainArguments,
    pub cameras_test: Cameras,
    pub cameras_train: Cameras,
    pub scene: Gaussian3dScene<Autodiff<Wgpu>>,
    pub trainer: Gaussian3dTrainer<Autodiff<Wgpu>>,
}

impl TrainArguments {
    pub fn init(&self) -> Result<TrainRunner, Report> {
        let arguments = self.to_owned();

        // Loading the cameras and points

        let (cameras_test, cameras_train, points) =
            get_cameras_and_points(&self.common_arguments)?;

        // Initializing the scene and trainer

        let device = WgpuDevice::default();
        let scene = Gaussian3dScene::from_points(points, &device);
        let trainer = Gaussian3dTrainerConfig::from(self).init(&device);

        Ok(TrainRunner {
            arguments,
            cameras_test,
            cameras_train,
            scene,
            trainer,
        })
    }
}

impl TrainRunner {
    pub fn run(mut self) -> Result<(), Report> {
        // TODO: RZC
        // - RZC_?

        // Specifying the parameters

        let device = self.scene.device();
        let iterations = self.arguments.iterations as usize;

        let mut iterations_test_reversed = self.arguments.test_iterations.to_owned();
        iterations_test_reversed.sort_unstable_by(|a, b| b.cmp(a));
        let mut iterations_save_reversed = self.arguments.save_iterations.to_owned();
        iterations_save_reversed.sort_unstable_by(|a, b| b.cmp(a));

        let metric_psnr = Psnr::init(&device);
        let range_detail_update = RangeOptions::default_with_step(
            2 * 100.max(self.trainer.refiner.config.range_densification.step),
        );
        let quiet = self.arguments.common_arguments.quiet;

        // Specifying the progress bar

        let mut bar = get_bar();
        let mut psnr = 0.0;
        let mut size = "0.0 B".to_string();
        bar.colour = Some("ansi(41)".into());
        bar.desc = "| Training 3DGS".into();
        bar.disable = quiet > 1;
        if quiet < 1 {
            bar.postfix = format!(" {size} | PSNR {psnr:.2} dB |");
        }
        bar.total = iterations;

        // Optimizing the scene iteratively

        self.cameras_train
            .seed(SEED)
            .random_values()
            .take(iterations)
            .try_for_each(|camera| {
                self.trainer.train(&mut self.scene, camera)?;
                let iteration = self.trainer.iteration;
                bar.update(1)?;

                // Updating the progress details
                if quiet < 1 && range_detail_update.has(iteration) {
                    let scene = self.scene.valid();
                    let output = scene
                        .render(&camera.view, &self.trainer.options_renderer)?
                        .colors_rgb_2d;
                    let target = camera.image.decode_rgb_tensor(&device)?;
                    psnr = metric_psnr.evaluate(output, target).into_scalar();
                    size = self.scene.size_readable();

                    bar.postfix = format!(" {size} | PSNR {psnr:.2} dB |");
                    bar.refresh()?;
                }

                // Testing the model
                if quiet < 1 && Some(&iteration) == iterations_test_reversed.last() {
                    iterations_test_reversed.pop();

                    let time = std::time::Instant::now();
                    let (mssim, psnr) = get_mssim_and_psnr(
                        &self.cameras_test,
                        &self.trainer.options_renderer,
                        &self.scene.valid(),
                    )?;
                    eprintln!("time: {:?}", time.elapsed());

                    bar.refresh()?;
                    eprintln!(
                        "|  Testing 3DGS | {size} | \
                        PSNR {psnr:.2} dB | SSIM {mssim:.3} |"
                    );
                }

                // Saving the model
                if Some(&iteration) == iterations_save_reversed.last() {
                    iterations_save_reversed.pop();

                    let model_path = &self.arguments.common_arguments.model_path;
                    Self::save_model(iteration, model_path, &self.scene.valid())?;
                    size = self.scene.size_readable();

                    bar.refresh()?;
                    eprintln!("|   Saving 3DGS | {size} |");
                }

                Ok::<_, Report>(())
            })?;

        if !bar.disable {
            if bar.should_refresh() {
                bar.refresh()?;
            }
            eprintln!();
        }

        Ok(())
    }

    pub fn save_model(
        iteration: u64,
        model_path: impl AsRef<Path>,
        scene: &Gaussian3dScene<Wgpu>,
    ) -> Result<PathBuf, Report> {
        let model_directory = [
            model_path.as_ref(),
            "point_cloud".as_ref(),
            format!("iteration_{iteration}").as_ref(),
        ]
        .iter()
        .collect::<PathBuf>();
        fs::create_dir_all(&model_directory)?;

        let model_file_path = model_directory.join("point_cloud.ply");
        scene.encode_polygon(File::open(&model_file_path)?.truncate()?)?;

        Ok(model_file_path)
    }
}

impl fmt::Debug for TrainRunner {
    #[inline]
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.debug_struct("TrainRunner")
            .field("arguments", &self.arguments)
            .field("cameras_test.len()", &self.cameras_test.len())
            .field("cameras_train.len()", &self.cameras_train.len())
            .field("scene", &self.scene)
            .field("trainer", &self.trainer)
            .finish()
    }
}

impl From<&TrainArguments> for Gaussian3dTrainerConfig {
    fn from(arguments: &TrainArguments) -> Self {
        Self::new()
            .with_learning_rate_colors_sh(LearningRateConfig::new(arguments.feature_lr))
            .with_learning_rate_opacities(LearningRateConfig::new(arguments.opacity_lr))
            .with_learning_rate_positions(
                LearningRateConfig::new(arguments.position_lr_init)
                    .with_end(arguments.position_lr_final)
                    .with_count(arguments.position_lr_max_steps),
            )
            .with_learning_rate_rotations(LearningRateConfig::new(arguments.rotation_lr))
            .with_learning_rate_scalings(LearningRateConfig::new(arguments.scaling_lr))
            .with_options_renderer(
                Gaussian3dRenderOptions::new()
                    .with_colors_sh_degree_max(arguments.sh_degree),
            )
            .with_range_metric_optimization_fine(RangeOptions::default_with_step(
                (arguments.lambda_dssim * 2.0).recip().max(1.0) as u64,
            ))
            .with_refiner(
                RefinerConfig::new()
                    .with_range_densification(RangeOptions::new(
                        arguments.densify_from_iter,
                        arguments.densify_until_iter,
                        arguments.densification_interval,
                    ))
                    .with_threshold_position_2d_grad_norm(
                        arguments.densify_grad_threshold,
                    )
                    .with_threshold_scaling(arguments.percent_dense * 4.0)
                    .with_range_increasing_colors_sh_degree_max(RangeOptions::new(
                        arguments.increase_sh_degree_from_iter,
                        arguments.increase_sh_degree_until_iter,
                        arguments.increase_sh_degree_interval,
                    )),
            )
    }
}
