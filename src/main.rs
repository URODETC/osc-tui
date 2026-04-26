use clap::{ Parser, ValueEnum};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl FromStr for Rgb {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "green" => Ok(Rgb(0, 255, 0)),
            "amber" => Ok(Rgb(255, 176, 0)),
            "cyan" => Ok(Rgb(0, 255, 255)),
            "white" => Ok(Rgb(255, 255, 255)),
            hex if hex.starts_with("#") && hex.len() == 7 => {
                let r = u8::from_str_radix(&hex[1..3], 16).map_err(|_| "Wrong Hex format (R)")?;
                let g = u8::from_str_radix(&hex[1..3], 16).map_err(|_| "Wrong Hex format (G)")?;
                let b = u8::from_str_radix(&hex[1..3], 16).map_err(|_| "Wrong Hex format (B)")?;
                Ok(Rgb(r,g,b))
            }
            _ => Err(format!("Unknown color: '{}'.", s))
        }
    }
}

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

#[derive(Parser, Debug)]
#[command(author, version, about = "CRT Oscilloscope TUI Screensaver", long_about = None)]
pub struct Cli {
    #[arg(short, long, value_enum, default_value_t = Mode::Lissajous)]
    pub mode: Mode,

    #[arg(short, long, value_enum, default_value_t = Renderer::Braille)]
    pub renderer: Renderer,

    #[arg(short, long, default_value = "green")]
    pub color: Rgb,

    #[arg(short, long, default_value_t = 0.05)]
    pub decay: f32,
}

fn main() {
    let cli = Cli::parse();

    println!("Config loaded:");
    println!("Mode: {:?}", cli.mode);
    println!("Renderer: {:?}", cli.renderer);
    println!("Color: {:?}", cli.color);
    println!("Decay speed: {}", cli.decay);
}
