mod app;
mod cli;
mod framebuffer;
mod palette;
mod render;
mod signals;
mod tui;

use app::App;
use clap::Parser;
use cli::Cli;
use std::io::Result;

fn main() -> Result<()> {
    let cli = Cli::parse();
    tui::install_panic_hook();
    tui::setup()?;

    let mut app = App::new(cli)?;
    let run_result = app.run();

    tui::restore()?;
    run_result
}
