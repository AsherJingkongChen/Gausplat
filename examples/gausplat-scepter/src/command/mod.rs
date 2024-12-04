pub mod gaussian_3d;

pub use super::*;
pub use clap::{builder::styling, Command, FromArgMatches, Parser, ValueEnum};
pub use color_eyre::Report;
pub use gaussian_3d::{Gaussian3dCommonArguments, Gaussian3dModelCommand};
pub use serde::{Deserialize, Serialize};

use clap::CommandFactory;
use gausplat_loader::source::file::{File, Opener};
use std::{
    fs::create_dir_all,
    io::BufReader,
    path::{Path, PathBuf},
};

pub const AFTER_HELP: &str = "\
    For more information, please see 'https://github.com/AsherJingkongChen/Gausplat'.";

pub const HELP_TEMPLATE: &str = "\
    {about}\n\n{usage-heading} {usage}\n\n{all-args}{after-help}";

pub const STYLES: styling::Styles = styling::Styles::styled()
    .header(styling::AnsiColor::Green.on_default().bold())
    .usage(styling::AnsiColor::Green.on_default().bold())
    .literal(styling::AnsiColor::Cyan.on_default().bold())
    .placeholder(styling::AnsiColor::Cyan.on_default())
    .error(styling::AnsiColor::Red.on_default().bold())
    .invalid(styling::AnsiColor::Yellow.on_default().bold())
    .valid(styling::AnsiColor::Cyan.on_default().bold());

/// Run tasks for Gausplat.
#[derive(Clone, Debug, Deserialize, Parser, PartialEq, Serialize)]
#[command(verbatim_doc_comment, rename_all = "kebab-case", after_help = AFTER_HELP)]
#[command(next_line_help = false, propagate_version = true, version)]
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
    /// Run tasks for the arguments.
    // #[arg(long, short = 'A', value_name = "Path")]
    #[command(verbatim_doc_comment, rename_all = "kebab-case", after_help = AFTER_HELP)]
    #[command(visible_alias = "r")]
    #[serde(rename = "inherit")]
    Run {
        /// Arguments file path.
        /// It is a JSON file containing the entire arguments.
        #[arg(verbatim_doc_comment, rename_all = "kebab-case")]
        #[arg(index = 1, value_name = "Path")]
        path: PathBuf,
    },

    /// Run tasks for 3D Gaussian Splatting.
    #[command(verbatim_doc_comment, rename_all = "kebab-case", after_help = AFTER_HELP)]
    #[command(subcommand, name = "3dgs")]
    #[serde(rename = "3dgs")]
    Gaussian3d(Gaussian3dModelCommand),
}

// TODO: Load arguments from a file.

impl GausplatArguments {
    #[inline]
    pub fn command() -> Command {
        <Self as CommandFactory>::command().styles(STYLES)
    }

    #[inline]
    pub fn load(file_path: impl AsRef<Path>) -> Result<Self, Report> {
        Ok(serde_json::from_reader(&mut BufReader::new(File::open(
            file_path.as_ref(),
        )?))?)
    }

    #[inline]
    pub fn parse() -> Result<Self, Report> {
        Ok(Self::from_arg_matches_mut(
            &mut Self::command().get_matches(),
        )?)
    }

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
