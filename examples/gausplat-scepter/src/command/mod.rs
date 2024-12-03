pub mod gaussian_3d;

pub use super::*;
pub use clap::{builder::styling, CommandFactory, FromArgMatches, Parser, ValueEnum};
pub use color_eyre::Report;
pub use gaussian_3d::{Gaussian3dCommonArguments, Gaussian3dModelCommand};
pub use serde::{Deserialize, Serialize};

use gausplat_loader::source::file::{File, Opener};
use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};

pub const AFTER_HELP: &str = "\
    For more information, please see 'https://github.com/AsherJingkongChen/Gausplat'.";
pub const HELP_TEMPLATE: &str = "\
    {about}\n\n{usage-heading} {usage}\n\n{all-args}{after-help}";

/// Run tasks for Gausplat.
#[derive(Clone, Debug, Deserialize, Parser, PartialEq, Serialize)]
#[command(verbatim_doc_comment, rename_all = "kebab-case", after_help = AFTER_HELP)]
#[command(propagate_version = true, version)]
pub struct GausplatArguments {
    /// Run tasks for the specific model.
    #[command(subcommand)]
    #[serde(flatten)]
    pub model: ModelCommand,
}

/// Run tasks for the specific model.
#[derive(Clone, Debug, Deserialize, PartialEq, Parser, Serialize)]
#[command(verbatim_doc_comment, rename_all = "kebab-case", after_help = AFTER_HELP)]
pub enum ModelCommand {
    /// Run tasks for 3D Gaussian Splatting.
    #[command(verbatim_doc_comment, rename_all = "kebab-case", after_help = AFTER_HELP)]
    #[command(subcommand, name = "3dgs")]
    #[serde(rename = "3dgs")]
    Gaussian3d(Gaussian3dModelCommand),
}

impl GausplatArguments {
    pub fn save(
        &self,
        directory: impl AsRef<Path>,
        file_stem: impl AsRef<Path>,
    ) -> Result<PathBuf, Report> {
        let directory = directory.as_ref();
        create_dir_all(directory)?;

        let args_file_path = directory.join(file_stem.as_ref().with_extension("json"));
        serde_json::to_writer_pretty(File::open(&args_file_path)?.truncate()?, self)?;

        let command = Self::command();
        if let Some(version) = command.get_version() {
            let mut version_file = File::open(directory.join("version.txt"))?;
            version_file.truncate()?;
            version_file.write_all(command.get_name().as_ref())?;
            version_file.write_all(b" ")?;
            version_file.write_all(version.as_ref())?;
        }

        Ok(args_file_path)
    }
}
