//! Evaluation command for 3DGS.

pub use super::*;

/// Evaluate for 3DGS.
#[derive(Clone, Debug, Deserialize, Parser, PartialEq, Serialize)]
#[command(verbatim_doc_comment, rename_all = "snake_case", after_help = AFTER_HELP)]
#[command(next_line_help = true)]
pub struct EvalArguments {
    /// Model directory paths.
    /// It may include files other than the model file.
    #[arg(verbatim_doc_comment, rename_all = "snake_case")]
    #[arg(long, short, value_name = "Path", num_args = 1..)]
    pub model_paths: Vec<PathBuf>,
}
