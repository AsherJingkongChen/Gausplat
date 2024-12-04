pub use gausplat_scepter::{
    command::{GausplatArguments, Gaussian3dModelCommand, ModelCommand, Report},
    runner::Runner,
};
use std::time::Instant;

pub fn main() -> Result<(), Report> {
    use ModelCommand::*;

    init()?;

    let args = GausplatArguments::parse()?;
    let args = match &args.model {
        Run { path } => GausplatArguments::load(path)?,
        _ => args,
    };

    log::debug!(target: "gausplat::scepter::main", "args > {args:#?}");

    let time = Instant::now();
    let log_runner = |runner: &dyn Runner| {
        let time = time.elapsed();
        log::debug!(target: "gausplat::scepter::main", "runner > {runner:#?}");
        log::info!(target: "gausplat::scepter::main", "run after {time:.03?}");
    };

    match &args.model {
        Gaussian3d(command) => {
            use Gaussian3dModelCommand::*;
            match command {
                Train(args_train) => {
                    args.save(&args_train.common_arguments.model_path, "args-train")?;
                    let runner = args_train.init()?;
                    log_runner(&runner);
                    runner.run()?;
                },
                Render(args_render) => {
                    args.save(&args_render.common_arguments.model_path, "args-render")?;
                    let runner = args_render.init()?;
                    log_runner(&runner);
                    runner.run()?;
                },
                Eval(_args_eval) => unimplemented!(),
            }
        },
        _ => unimplemented!(),
    };

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
