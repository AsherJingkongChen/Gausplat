pub use super::*;

use gausplat_trainer::train::gaussian_3d::Gaussian3dTrainerConfig;
use gausplat_trainer::train::gaussian_3d::RefinerConfig;
use std::sync::LazyLock;

/// Train for 3D Gaussian Splatting.
#[derive(Clone, Debug, Deserialize, Parser, PartialEq, Serialize)]
#[command(verbatim_doc_comment, rename_all = "kebab-case", after_help = AFTER_HELP)]
#[command(next_line_help = true)]
pub struct TrainArguments {
    /// Color SH feature degree at initial.
    /// It generally ranges from 0 to 3.
    #[arg(verbatim_doc_comment, rename_all = "kebab-case")]
    #[arg(long, value_name = "U32", default_value_t = 0)]
    pub sh_degree: u32,

    /// Common arguments for 3D Gaussian Splatting.
    #[command(flatten)]
    pub common_arguments: Gaussian3dCommonArguments,

    /// Iterations for testing.
    /// It may take few second to run.
    /// If '-qq' is set, no test will be performed.
    #[arg(verbatim_doc_comment, rename_all = "kebab-case")]
    #[arg(
        long, value_name = "U64", num_args = 0..,
        default_values_t = vec![7000, 30000],
    )]
    pub test_iterations: Vec<u64>,

    /// Iterations for saving.
    /// It may take few second to run.
    /// If '-qqq' is set, no message will be shown.
    #[arg(verbatim_doc_comment, rename_all = "kebab-case")]
    #[arg(
        long, value_name = "U64", num_args = 0..,
        default_values_t = vec![7000, 30000],
    )]
    pub save_iterations: Vec<u64>,

    /// Number of iterations for training.
    /// It is recommended to set a larger value for larger datasets.
    #[arg(verbatim_doc_comment, rename_all = "kebab-case")]
    #[arg(long, value_name = "U64", default_value_t = 30000)]
    pub iterations: u64,

    /// Learning rate at initial for position.
    #[arg(verbatim_doc_comment, rename_all = "kebab-case")]
    #[arg(
        long, value_name = "F64",
        default_value_t = TRAINER_CONFIG.learning_rate_positions.start,
    )]
    pub position_lr_init: f64,

    /// Learning rate at final for position.
    #[arg(verbatim_doc_comment, rename_all = "kebab-case")]
    #[arg(
        long, value_name = "F64",
        default_value_t = TRAINER_CONFIG.learning_rate_positions.end,
    )]
    pub position_lr_final: f64,

    /// Learning rate maximum step count for position.
    /// It should be equal to the number of iterations.
    #[arg(verbatim_doc_comment, rename_all = "kebab-case")]
    #[arg(
        long, value_name = "U64",
        default_value_t = TRAINER_CONFIG.learning_rate_positions.count,
    )]
    pub position_lr_max_steps: u64,

    /// Learning rate for color SH feature.
    #[arg(verbatim_doc_comment, rename_all = "kebab-case")]
    #[arg(
        long, value_name = "F64",
        default_value_t = TRAINER_CONFIG.learning_rate_colors_sh.start,
    )]
    pub feature_lr: f64,

    /// Learning rate for opacity.
    #[arg(verbatim_doc_comment, rename_all = "kebab-case")]
    #[arg(
        long, value_name = "F64",
        default_value_t = TRAINER_CONFIG.learning_rate_opacities.start,
    )]
    pub opacity_lr: f64,

    /// Learning rate for scaling.
    #[arg(verbatim_doc_comment, rename_all = "kebab-case")]
    #[arg(
        long, value_name = "F64",
        default_value_t = TRAINER_CONFIG.learning_rate_scalings.start,
    )]
    pub scaling_lr: f64,

    /// Learning rate for rotation.
    #[arg(verbatim_doc_comment, rename_all = "kebab-case")]
    #[arg(
        long, value_name = "F64",
        default_value_t = TRAINER_CONFIG.learning_rate_rotations.start,
    )]
    pub rotation_lr: f64,

    /// Tolerance for scalings.
    /// It may affect the overall quality and model size.
    #[arg(verbatim_doc_comment, rename_all = "kebab-case")]
    #[arg(
        long, value_name = "F64",
        default_value_t = REFINER_CONFIG.threshold_scaling / 4.0,
    )]
    pub percent_dense: f64,

    /// Usage of the structural dissimilarity index.
    /// It may affect the training quality, time, and model size.
    #[arg(verbatim_doc_comment, rename_all = "kebab-case")]
    #[arg(
        long, value_name = "F64",
        default_value_t =
            (TRAINER_CONFIG.range_metric_optimization_fine.step as f64 * 2.0).recip(),
    )]
    pub lambda_dssim: f64,

    /// Number of iterations between densification.
    /// It may affect the training quality and model size.
    /// It should not be too small.
    #[arg(verbatim_doc_comment, rename_all = "kebab-case")]
    #[arg(
        long, value_name = "U64",
        default_value_t = REFINER_CONFIG.range_densification.step,
    )]
    pub densification_interval: u64,

    // TODO: `opacity_reset_interval`. We require more tests for opacity resetting.
    /// The start iteration for densification.
    /// It should not be too small.
    #[arg(verbatim_doc_comment, rename_all = "kebab-case")]
    #[arg(
        long, value_name = "U64",
        default_value_t = REFINER_CONFIG.range_densification.start,
    )]
    pub densify_from_iter: u64,

    /// The final iteration for densification.
    /// No action will be performed at and after this iteration.
    #[arg(verbatim_doc_comment, rename_all = "kebab-case")]
    #[arg(
        long, value_name = "U64",
        default_value_t = REFINER_CONFIG.range_densification.end,
    )]
    pub densify_until_iter: u64,

    /// Tolerance for view-space positional error.
    /// It may affect the training quality and model size.
    #[arg(verbatim_doc_comment, rename_all = "kebab-case")]
    #[arg(
        long, value_name = "F64",
        default_value_t = REFINER_CONFIG.threshold_position_2d_grad_norm,
    )]
    pub densify_grad_threshold: f64,

    /// Number of iterations between increasing color SH feature degree by one.
    /// It may affect the training time.
    #[arg(verbatim_doc_comment, rename_all = "kebab-case")]
    #[arg(
        long, value_name = "U64",
        default_value_t = REFINER_CONFIG.range_increasing_colors_sh_degree_max.step,
    )]
    pub increase_sh_degree_interval: u64,

    /// The start iteration for increasing color SH feature degree by one.
    #[arg(verbatim_doc_comment, rename_all = "kebab-case")]
    #[arg(
        long, value_name = "U64",
        default_value_t =
            REFINER_CONFIG.range_increasing_colors_sh_degree_max.start,
    )]
    pub increase_sh_degree_from_iter: u64,

    /// The final iteration for increasing color SH feature degree by one.
    /// No action will be performed at and after this iteration.
    #[arg(verbatim_doc_comment, rename_all = "kebab-case")]
    #[arg(
        long, value_name = "U64",
        default_value_t =
            REFINER_CONFIG.range_increasing_colors_sh_degree_max.end,
    )]
    pub increase_sh_degree_until_iter: u64,
}

/// [`Gaussian3dTrainerConfig::default()`]
pub static TRAINER_CONFIG: LazyLock<Gaussian3dTrainerConfig> =
    LazyLock::new(Gaussian3dTrainerConfig::default);

/// [`Gaussian3dTrainerConfig::default()`]
pub static REFINER_CONFIG: LazyLock<RefinerConfig> =
    LazyLock::new(|| Gaussian3dTrainerConfig::default().refiner);
