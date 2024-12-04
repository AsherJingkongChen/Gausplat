pub mod gaussian_3d;

pub use super::*;
pub use color_eyre::Report;
pub use kdam::{Bar, BarBuilder, BarExt};

use std::fmt::Debug;

/// Task runner.
pub trait Runner: Debug {
    fn run(self) -> Result<(), Report>;
}

pub fn get_bar() -> Bar {
    // NOTE: The bar format is correct.
    let bar = kdam::BarBuilder::default()
        .dynamic_ncols(true)
        .unit("")
        .mininterval(0.1)
        .miniters(2)
        .bar_format(
            "{desc suffix=''} |{postfix} \
            {percentage:.1}% ({count}/{total}, {rate:.2}/s) | \
            {elapsed human=true} < {remaining human=true} \
            |{animation}|",
        )
        .build()
        .unwrap();
    init_bar(&bar);
    bar
}

pub fn init_bar(bar: &Bar) {
    use kdam::term::{init, InitializedOutput::*};
    use std::io::{stderr, stdout, IsTerminal};

    init(match bar.writer {
        Stderr => stderr().is_terminal(),
        Stdout => stdout().is_terminal(),
        _ => false,
    });
}
