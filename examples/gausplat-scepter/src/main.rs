use gausplat_scepter::command::{
    styling, CommandFactory, FromArgMatches, GausplatArguments, Gaussian3dModelCommand,
    ModelCommand,
};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    std::env::set_var("RUST_BACKTRACE", "full");

    pretty_env_logger::formatted_timed_builder()
        .parse_default_env()
        .try_init()?;
    log::info!(target: "gausplat::dev", "main");

    let styles = styling::Styles::styled()
        .header(styling::AnsiColor::Green.on_default().bold())
        .usage(styling::AnsiColor::Green.on_default().bold())
        .literal(styling::AnsiColor::Cyan.on_default().bold())
        .placeholder(styling::AnsiColor::Cyan.on_default())
        .error(styling::AnsiColor::Red.on_default().bold())
        .invalid(styling::AnsiColor::Yellow.on_default().bold())
        .valid(styling::AnsiColor::Cyan.on_default().bold());
    let args = GausplatArguments::from_arg_matches_mut(
        &mut GausplatArguments::command().styles(styles).get_matches(),
    )?;

    use ModelCommand::*;
    match &args.model {
        Gaussian3d(command) => {
            use Gaussian3dModelCommand::*;
            match command {
                Train(args_train) => {
                    args.save(&args_train.common_arguments.model_path, "args-train")?;
                    args_train.init()?.run()?;
                },
                Render(args_render) => {
                    // args-render is not needed.
                    args.save(&args_render.common_arguments.model_path, "args-render")?;
                    args_render.init()?.run()?;
                },
                Eval(_args_eval) => unimplemented!(),
            }
        },
    }

    Ok(())
}
