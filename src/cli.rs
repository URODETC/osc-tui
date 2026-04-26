use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, Eq, PartialEq, ValueEnum, Debug)]
pub enum Mode {
    Lissajous,
    AudioXy,
    AudioYt,
}

#[derive(Copy, Clone, Eq, PartialEq, ValueEnum, Debug)]
pub enum Renderer {
    Braille,
    Block,
}

#[derive(Copy, Clone, Eq, PartialEq, ValueEnum, Debug)]
pub enum Palette {
    Green,
    Amber,
    Red,
    Cyan,
    Ghost,
}

#[derive(Parser, Debug)]
#[command(author, version, about = "CRT Oscilloscope TUI Screensaver", long_about = None)]
pub struct Cli {
    #[arg(short, long, value_enum, default_value_t = Mode::Lissajous)]
    pub mode: Mode,

    #[arg(short, long, value_enum, default_value_t = Renderer::Braille)]
    pub renderer: Renderer,

    #[arg(short, long, value_enum, default_value_t = Palette::Green)]
    pub palette: Palette,

    #[arg(short, long, default_value_t = 0.05)]
    pub decay: f32,
}
