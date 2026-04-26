pub(crate) use crate::cli::Palette;

#[derive(Copy, Clone, PartialEq)]
pub struct Rgb(pub u8, pub u8, pub u8);

struct ColorStops {
    hot: (f32, f32, f32),
    bright: (f32, f32, f32),
    normal: (f32, f32, f32),
    dim: (f32, f32, f32),
}

impl Palette {
    fn stops(&self) -> ColorStops {
        match self {
            Palette::Green => ColorStops {
                hot: (220.0, 255.0, 220.0),
                bright: (81.0, 255.0, 50.0),
                normal: (41.0, 180.0, 0.0),
                dim: (0.0, 50.0, 0.0),
            },
            Palette::Amber => ColorStops {
                hot: (255.0, 255.0, 200.0),
                bright: (255.0, 176.0, 0.0),
                normal: (180.0, 100.0, 0.0),
                dim: (60.0, 30.0, 0.0),
            },
            Palette::Red => ColorStops {
                hot: (255.0, 200.0, 200.0),
                bright: (255.0, 50.0, 50.0),
                normal: (180.0, 0.0, 0.0),
                dim: (50.0, 0.0, 0.0),
            },
            Palette::Cyan => ColorStops {
                hot: (200.0, 255.0, 255.0),
                bright: (0.0, 255.0, 255.0),
                normal: (0.0, 150.0, 150.0),
                dim: (0.0, 50.0, 50.0),
            },
            Palette::Ghost => ColorStops {
                hot: (255.0, 255.0, 255.0),
                bright: (180.0, 200.0, 255.0),
                normal: (100.0, 120.0, 180.0),
                dim: (30.0, 40.0, 60.0),
            },
        }
    }

    pub fn get_color(&self, brightness: f32) -> Rgb {
        if brightness <= 0.01 {
            return Rgb(0, 0, 0);
        }

        let stops = self.stops();
        let black = (0.0, 0.0, 0.0);

        let (c1, c2, t) = if brightness > 0.6 {
            let t = (brightness - 0.6) / 0.4;
            (stops.bright, stops.hot, t)
        } else if brightness > 0.3 {
            let t = (brightness - 0.3) / 0.3;
            (stops.normal, stops.bright, t)
        } else if brightness > 0.1 {
            let t = (brightness - 0.1) / 0.1;
            (stops.dim, stops.normal, t)
        } else {
            let t = brightness / 0.1;
            (black, stops.dim, t)
        };

        let r = lerp(c1.0, c2.0, t);
        let g = lerp(c1.1, c2.1, t);
        let b = lerp(c1.2, c2.2, t);

        Rgb(r as u8, g as u8, b as u8)
    }
}
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t.clamp(0.0, 1.0)
}
