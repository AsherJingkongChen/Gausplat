//! Command for 3DGS.

pub mod eval;
pub mod render;
pub mod train;

pub use super::*;
pub use eval::*;
pub use render::*;
pub use train::*;

use clap::ArgAction;

/// Run tasks for 3DGS.
#[derive(Clone, Debug, Deserialize, PartialEq, Parser, Serialize)]
#[command(verbatim_doc_comment, rename_all = "snake_case", after_help = AFTER_HELP)]
pub enum Gaussian3dModelCommand {
    /// Evaluate for 3DGS.
    #[command(verbatim_doc_comment, rename_all = "snake_case", after_help = AFTER_HELP)]
    #[serde(rename = "eval")]
    Eval(EvalArguments),

    /// Render for 3DGS.
    #[command(verbatim_doc_comment, rename_all = "snake_case", after_help = AFTER_HELP)]
    #[serde(rename = "render")]
    Render(RenderArguments),

    /// Train for 3DGS.
    #[command(verbatim_doc_comment, rename_all = "snake_case", after_help = AFTER_HELP)]
    #[serde(rename = "train")]
    Train(TrainArguments),
}

/// Common arguments for 3DGS.
#[derive(Clone, Debug, Deserialize, Serialize, Parser, PartialEq)]
#[command(verbatim_doc_comment, rename_all = "snake_case", after_help = AFTER_HELP)]
pub struct Gaussian3dCommonArguments {
    /// Dataset directory path.
    /// It may include prior and image files.
    #[arg(verbatim_doc_comment, rename_all = "snake_case")]
    #[arg(long, short, value_name = "Path")]
    pub source_path: PathBuf,

    /// Dataset type.
    #[arg(verbatim_doc_comment, rename_all = "snake_case")]
    #[arg(long, short = 't', value_name = "Path", default_value = "colmap")]
    pub source_type: SourceType,

    /// Model directory path.
    /// It may include files other than the model file.
    #[arg(verbatim_doc_comment, rename_all = "snake_case")]
    #[arg(long, short, value_name = "Path")]
    pub model_path: PathBuf,

    /// Dataset image sub-directory path.
    /// It may be included in the dataset directory.
    #[arg(verbatim_doc_comment, rename_all = "snake_case")]
    #[arg(long, short, value_name = "Path", default_value = "images")]
    pub images: PathBuf,

    // /// Enabling the white background.
    // #[arg(verbatim_doc_comment, rename_all = "snake_case")]
    // TODO: #[arg(long, default_value_t = false)]
    // pub white_background: bool,
    /// Enabling evaluation mode.
    /// It should be set if running evaluation tasks is required.
    #[arg(verbatim_doc_comment, rename_all = "snake_case")]
    #[arg(long, short, default_value_t = false)]
    pub eval: bool,

    /// Disabling messages.
    /// The more quiet flags are set, the less messages are shown.
    #[arg(verbatim_doc_comment, rename_all = "snake_case")]
    #[arg(long, short, action = ArgAction::Count, default_value_t = 0)]
    pub quiet: u8,
}

/// Dataset type.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, ValueEnum)]
#[value(verbatim_doc_comment, rename_all = "snake_case")]
pub enum SourceType {
    /// The dataset is built from COLMAP.
    /// It generally includes the following files:
    /// 1. sparse/0/cameras.bin
    /// 2. sparse/0/images.bin
    /// 3. sparse/0/points3D.bin
    /// 4. images/*.jpg
    #[default]
    #[value(verbatim_doc_comment)]
    #[serde(rename = "colmap")]
    Colmap,
}
