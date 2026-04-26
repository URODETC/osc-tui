pub struct FrameBuffer {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<f32>,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![0.0; width * height],
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.pixels = vec![0.0; width * height];
    }

    pub fn set_pixel(&mut self, x: isize, y: isize, brightness: f32) {
        if x >= 0 && x < self.width as isize && y < self.height as isize {
            let idx = (y as usize) * self.width + (x as usize);
            self.pixels[idx] = brightness.clamp(0.0, 1.1);
        }
    }

    pub fn apply_decay(&mut self, decay_rate: f32) {
        let multiplier = 1.0 - decay_rate;
        for pixel in self.pixels.iter_mut() {
            if *pixel > 0.0 {
                *pixel *= multiplier;
                if *pixel < 0.01 {
                    *pixel = 0.0;
                }
            }
        }
    }

    pub fn draw_line(&mut self, x0: isize, y0: isize, x1: isize, y1: isize, brightness: f32) {
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        let mut x = x0;
        let mut y = y0;

        loop {
            self.set_pixel(x, y, brightness);
            if x == x1 && y == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    pub fn map_coords(&mut self, mut x: f32, y: f32) -> (isize, isize) {
        let aspect_ratio = 2.0;
        x *= aspect_ratio;
        let min_dim = (self.width as f32 / aspect_ratio).min(self.height as f32) / 2.0;

        let cx = self.width as f32 / 2.0;
        let cy = self.height as f32 / 2.0;

        let scr_x = cx + (x * min_dim);
        let scr_y = cy + (y * min_dim);

        (scr_x.round() as isize, scr_y.round() as isize)
    }
}
