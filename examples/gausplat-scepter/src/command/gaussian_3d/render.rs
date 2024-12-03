pub use super::*;

use gausplat_renderer::render::gaussian_3d::Gaussian3dRenderOptions;

/// Render for 3D Gaussian Splatting.
#[derive(Clone, Debug, Deserialize, Parser, PartialEq, Serialize)]
#[command(verbatim_doc_comment, rename_all = "kebab-case", after_help = AFTER_HELP)]
#[command(next_line_help = true)]
pub struct RenderArguments {
    /// Iteration for rendering.
    /// It refers to the model saving iteration.
    /// [default: Maximum saving iteration]
    #[arg(verbatim_doc_comment, rename_all = "kebab-case")]
    #[arg(long, value_name = "U64")]
    pub iteration: Option<u64>,

    /// Do not render the training dataset.
    #[arg(verbatim_doc_comment, rename_all = "kebab-case")]
    #[arg(long, default_value_t = false)]
    pub skip_train: bool,

    /// Do not render the testing dataset.
    #[arg(verbatim_doc_comment, rename_all = "kebab-case")]
    #[arg(long, default_value_t = false)]
    pub skip_test: bool,

    /// Color SH feature degree.
    /// It generally ranges from 0 to 3.
    #[arg(verbatim_doc_comment, rename_all = "kebab-case")]
    #[arg(
        long, value_name = "U32",
        default_value_t = Gaussian3dRenderOptions::new().colors_sh_degree_max,
    )]
    pub sh_degree: u32,

    /// Common arguments for 3D Gaussian Splatting.
    #[command(flatten)]
    pub common_arguments: Gaussian3dCommonArguments,
}
