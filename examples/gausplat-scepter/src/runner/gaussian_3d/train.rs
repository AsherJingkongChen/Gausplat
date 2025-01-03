//! Training runner for 3DGS.

pub use super::*;
pub use command::gaussian_3d::TrainArguments;
pub use gausplat::trainer::train::gaussian_3d::Gaussian3dTrainerConfig;

use gausplat::renderer::spherical_harmonics::SH_DEGREE_MAX;
use gausplat::trainer::{
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
    time::Instant,
};

/// Training runner.
#[derive(Clone)]
pub struct TrainRunner {
    /// Arguments for training.
    pub arguments: TrainArguments,
    /// Cameras for testing.
    pub cameras_test: Cameras,
    /// Cameras for training.
    pub cameras_train: Cameras,
    /// Scene to train.
    pub scene: Gaussian3dScene<Autodiff<Wgpu>>,
    /// Trainer.
    pub trainer: Gaussian3dTrainer<Autodiff<Wgpu>>,
}

impl TrainArguments {
    /// Initialize the training runner.
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
    /// Saves the model to the specified path.
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

impl Runner for TrainRunner {
    fn run(mut self) -> Result<(), Report> {
        // Specifying the parameters

        let device = self.scene.device();
        let iterations = self.arguments.iterations as usize;

        let mut iterations_test_reversed = self.arguments.test_iterations.to_owned();
        iterations_test_reversed.sort_unstable_by(|a, b| b.cmp(a));
        let mut iterations_save_reversed = self.arguments.save_iterations.to_owned();
        iterations_save_reversed.sort_unstable_by(|a, b| b.cmp(a));

        let metric_psnr = Psnr::init(&device);
        let range_detail_update = self.trainer.refiner.config.range_densification;
        let quiet = self.arguments.common_arguments.quiet;

        let can_show_details = quiet < 1;
        let can_show_test = quiet < 2;
        let can_show_save = quiet < 3;
        let can_show_size = quiet < 3;

        // Specifying the progress bar

        let mut bar = get_bar();
        let mut psnr = 0.0;
        let mut size = "0.0 B".to_string();
        bar.colour = Some("ansi(41)".into());
        bar.desc = "| Training 3DGS".into();
        bar.disable = quiet > 1;
        bar.total = iterations;
        if can_show_details {
            bar.postfix = format!(" {size} | PSNR {psnr:.2} dB |");
        }

        // Rescaling down the images at initialization

        let time = Instant::now();
        resize_cameras(&mut self.cameras_train)?;
        log::info!(
            target: "gausplat::scepter::gaussian_3d::train",
            "may rescale in {:.03?}", time.elapsed(),
        );

        // Optimizing the scene iteratively

        let result = self
            .cameras_train
            .seed(SEED)
            .random_values()
            .take(iterations)
            .try_for_each(|camera| {
                self.trainer.train(&mut self.scene, camera)?;
                bar.update(1)?;

                // Specifying the parameters
                let iteration = self.trainer.iteration;
                if can_show_size {
                    size = self.scene.size_readable();
                }

                // Updating the progress details
                if can_show_details && range_detail_update.has(iteration) {
                    let scene = self.scene.valid();
                    let output = scene
                        .render(&camera.view, &self.trainer.options_renderer)?
                        .colors_rgb_2d;
                    let target = camera.image.decode_rgb_tensor(&device)?;
                    psnr = metric_psnr.evaluate(output, target).into_scalar();
                    bar.postfix = format!(" {size} | PSNR {psnr:.2} dB |");
                    if !bar.disable {
                        bar.refresh()?;
                    }
                }

                // NOTE: The progress should be shown only once.
                let mut should_show_progress = true;

                // Testing the model
                if can_show_test && Some(&iteration) == iterations_test_reversed.last() {
                    iterations_test_reversed.pop();

                    let (mssim, psnr) = get_mssim_and_psnr(
                        &self.cameras_test,
                        &self.trainer.options_renderer,
                        &self.scene.valid(),
                    )?;

                    if should_show_progress {
                        should_show_progress = false;
                        bar.refresh()?;
                    }
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

                    if can_show_save {
                        if should_show_progress {
                            bar.refresh()?;
                        }
                        eprintln!("|   Saving 3DGS | {size} |");
                    }
                }

                Ok(())
            });

        if !bar.disable {
            eprintln!();
        }

        result
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
        let arguments_increasing_sh_degree_until_iter = arguments
            .increase_sh_degree_from_iter
            + arguments.increase_sh_degree_interval * SH_DEGREE_MAX as u64;

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
                (0.4 / arguments.lambda_dssim).max(1.0).round() as u64,
            ))
            .with_refiner(
                RefinerConfig::new()
                    .with_range_densification(RangeOptions::new(
                        arguments.densify_from_iter,
                        arguments.densify_until_iter,
                        arguments.densification_interval,
                    ))
                    .with_range_increasing_colors_sh_degree_max(RangeOptions::new(
                        arguments.increase_sh_degree_from_iter,
                        arguments_increasing_sh_degree_until_iter,
                        arguments.increase_sh_degree_interval,
                    ))
                    .with_threshold_opacity(arguments.prune_opacity_threshold)
                    .with_threshold_position_2d_grad_norm(
                        arguments.densify_grad_threshold,
                    )
                    .with_threshold_scaling((arguments.percent_dense * 5.0).max(0.0)),
            )
    }
}
