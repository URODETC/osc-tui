use crate::framebuffer::FrameBuffer;
use crate::palette::{Palette, Rgb};

#[derive(Clone, Copy, PartialEq)]
pub struct TermCell {
    pub ch: char,
    pub fg: Rgb,
}

impl Default for TermCell {
    fn default() -> Self {
        Self {
            ch: ' ',
            fg: Rgb(0, 0, 0),
        }
    }
}

pub struct TerminalBuffer {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<TermCell>,
}

impl TerminalBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![TermCell::default(); width * height],
        }
    }

    pub fn render_braille(&mut self, fb: &FrameBuffer, palette: &Palette) {
        let dot_map = [[0x01, 0x08], [0x02, 0x10], [0x04, 0x20], [0x40, 0x80]];

        for ty in 0..self.height {
            for tx in 0..self.width {
                let mut braille_bits = 0;
                let mut max_brightness = 0.0_f32;

                for dy in 0..4 {
                    for dx in 0..2 {
                        let px = tx * 2 + dx;
                        let py = ty * 4 + dy;

                        if px < fb.width && py < fb.height {
                            let brightness = fb.pixels[py * fb.width + px];
                            if brightness > 0.01 {
                                braille_bits |= dot_map[dy][dx];
                                if brightness > max_brightness {
                                    max_brightness = brightness
                                }
                            }
                        }
                    }
                }

                let ch = if braille_bits == 0 {
                    ' '
                } else {
                    char::from_u32(0x2800 + braille_bits).unwrap_or(' ')
                };

                let idx = ty * self.width + tx;
                self.cells[idx] = TermCell {
                    ch,
                    fg: palette.get_color(max_brightness),
                }
            }
        }
    }
}
