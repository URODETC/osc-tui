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

impl Mode {
    pub fn next(&self) -> Self {
        match self {
            Mode::Lissajous => Mode::AudioXy,
            Mode::AudioXy => Mode::AudioYt,
            Mode::AudioYt => Mode::Lissajous,
        }
    }
    pub fn prev(&self) -> Self {
        match self {
            Mode::Lissajous => Mode::AudioYt,
            Mode::AudioXy => Mode::Lissajous,
            Mode::AudioYt => Mode::AudioXy,
        }
    }
}

impl Palette {
    pub fn next(&self) -> Self {
        match self {
            Palette::Green => Palette::Amber,
            Palette::Amber => Palette::Red,
            Palette::Red => Palette::Cyan,
            Palette::Cyan => Palette::Ghost,
            Palette::Ghost => Palette::Green,
        }
    }
    pub fn prev(&self) -> Self {
        match self {
            Palette::Green => Palette::Ghost,
            Palette::Amber => Palette::Green,
            Palette::Red => Palette::Amber,
            Palette::Cyan => Palette::Red,
            Palette::Ghost => Palette::Cyan,
        }
    }
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

    #[arg(short, long, default_value_t = 0.04)]
    pub decay: f32,
}
