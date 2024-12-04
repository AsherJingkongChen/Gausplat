use gausplat_scepter::command::{
    GausplatArguments, Gaussian3dModelCommand, ModelCommand, Report,
};

pub fn main() -> Result<(), Report> {
    use ModelCommand::*;

    init()?;

    let args = GausplatArguments::parse()?;
    let args = match &args.model {
        Run { path } => GausplatArguments::load(path)?,
        _ => args,
    };

    #[cfg(all(debug_assertions, not(test)))]
    log::debug!(target: "gausplat-scepter::main", "args > {args:#?}");

    match &args.model {
        Gaussian3d(command) => {
            use Gaussian3dModelCommand::*;
            match command {
                Train(args_train) => {
                    args.save(&args_train.common_arguments.model_path, "args-train")?;
                    let runner = args_train.init()?;

                    // TODO: DRY: Runner::run

                    #[cfg(all(debug_assertions, not(test)))]
                    log::debug!(target: "gausplat::scepter::main", "runner > {runner:#?}");

                    runner.run()?;
                },
                Render(args_render) => {
                    args.save(&args_render.common_arguments.model_path, "args-render")?;
                    let runner = args_render.init()?;

                    #[cfg(all(debug_assertions, not(test)))]
                    log::debug!(target: "gausplat::scepter::main", "runner > {runner:#?}");

                    runner.run()?;
                },
                Eval(_args_eval) => unimplemented!(),
            }
        },
        _ => {},
    }

    Ok(())
}

pub fn init() -> Result<(), Report> {
    color_eyre::install()?;
    std::env::set_var("RUST_BACKTRACE", "full");

    pretty_env_logger::formatted_timed_builder()
        .parse_default_env()
        .try_init()?;

    log::info!(target: "gausplat::scepter::main", "init");

    Ok(())
}
