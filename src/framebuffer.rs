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
            self.pixels[idx] = (brightness).clamp(0.0, 1.0);
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

    pub fn draw_line(
        &mut self,
        mut x0: f32,
        mut y0: f32,
        mut x1: f32,
        mut y1: f32,
        brightness: f32,
    ) {
        let steep = (y1 - y0).abs() > (x1 - x0).abs();

        if steep {
            std::mem::swap(&mut x0, &mut y0);
            std::mem::swap(&mut x1, &mut y1);
        }
        if x0 > x1 {
            std::mem::swap(&mut x0, &mut x1);
            std::mem::swap(&mut y0, &mut y1);
        }

        let dx = x1 - x0;
        let dy = y1 - y0;
        let gradient = if dx == 0.0 { 1.0 } else { dy / dx };

        let mut intersect_y = y0;

        for x in (x0.round() as isize)..=(x1.round() as isize) {
            let y_int = intersect_y.trunc() as isize;
            let y_fract = intersect_y.fract();

            let intensity1 = (1.0 - y_fract) * brightness;
            let intensity2 = y_fract * brightness;

            if steep {
                self.set_pixel(y_int, x, intensity1);
                self.set_pixel(y_int + 1, x, intensity2);
            } else {
                self.set_pixel(x, y_int, intensity1);
                self.set_pixel(x, y_int + 1, intensity2);
            }
            intersect_y += gradient;
        }
    }

    pub fn map_coords(&mut self, mut x: f32, y: f32) -> (f32, f32) {
        let aspect_ratio = 2.0;
        x *= aspect_ratio;
        let min_dim = ((self.width as f32 / aspect_ratio).min(self.height as f32) - 2.0) / 2.0;

        let cx = self.width as f32 / 2.0;
        let cy = self.height as f32 / 2.0;

        let scr_x = cx + (x * min_dim);
        let scr_y = cy + (y * min_dim);

        (scr_x, scr_y)
    }
}
