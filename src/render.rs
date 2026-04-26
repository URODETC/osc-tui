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
                            if brightness > 0.06 {
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

    pub fn draw_text(&mut self, x: usize, y: usize, text: &str, fg: Rgb) {
        if y >= self.height {
            return;
        }
        for (i, ch) in text.chars().enumerate() {
            let px = x + i;
            if px < self.width {
                self.cells[y * self.width + px] = TermCell { ch, fg };
            }
        }
    }

    pub fn draw_box(&mut self, x: usize, y: usize, w: usize, h: usize, title: &str, fg: Rgb) {
        if x + h > self.width || y + h > self.height {
            return;
        }

        let (tl, tr, bl, br) = ('┌', '┐', '└', '┘');
        let (h_line, v_line) = ('─', '│');

        for i in 1..(w - 1) {
            self.cells[y * self.width + (x + i)] = TermCell { ch: h_line, fg };
            self.cells[(y + h - 1) * self.width + (x + i)] = TermCell { ch: h_line, fg };
        }

        for j in 1..(h - 1) {
            self.cells[(y + j) * self.width + x] = TermCell { ch: v_line, fg };
            self.cells[(y + j) * self.width + (x + w - 1)] = TermCell { ch: v_line, fg };
            for i in 1..(w - 1) {
                self.cells[(y + j) * self.width + (x + i)] = TermCell { ch: ' ', fg };
            }
        }
        self.cells[y * self.width + x] = TermCell { ch: tl, fg };
        self.cells[y * self.width + (x + w - 1)] = TermCell { ch: tr, fg };
        self.cells[(y + h - 1) * self.width + x] = TermCell { ch: bl, fg };
        self.cells[(y + h - 1) * self.width + (x + w - 1)] = TermCell { ch: br, fg };

        if !title.is_empty() {
            let title_str = format!(" {} ", title);
            self.draw_text(x + 2, y, &title_str, fg);
        }
    }
}
