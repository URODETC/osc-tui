use crate::cli::Mode;

pub trait SignalGenerator {
    fn next_sample(&mut self, time: f32) -> f32;
}

pub struct SineWave {
    pub frequency: f32,
    pub phase: f32,
}

impl SignalGenerator for SineWave {
    fn next_sample(&mut self, time: f32) -> f32 {
        ((time * self.frequency) + self.phase).sin()
    }
}

pub struct Oscilloscope {
    pub mode: Mode,
    pub x_gen: Box<dyn SignalGenerator>,
    pub y_gen: Box<dyn SignalGenerator>,
    pub time: f32,
}

impl Oscilloscope {
    pub fn new(mode: Mode) -> Self {
        Self {
            mode,
            x_gen: Box::new(SineWave {
                frequency: 3.0,
                phase: 0.0,
            }),
            y_gen: Box::new(SineWave {
                frequency: 2.0,
                phase: 0.0,
            }),
            time: 0.0,
        }
    }

    pub fn generate_chunk(&mut self, samples: usize, dt: f32) -> Vec<(f32, f32)> {
        let mut points = Vec::with_capacity(samples);
        let time_step = dt / samples as f32;

        for _ in 0..samples {
            self.time += time_step;

            match self.mode {
                Mode::AudioXy | Mode::Lissajous => {
                    let x = self.x_gen.next_sample(self.time);
                    let y = self.y_gen.next_sample(self.time);
                    points.push((x, y));
                }
                Mode::AudioYt => {
                    let sweep_freq = 0.5;
                    let x = (self.time * sweep_freq % 1.0) * 2.0 - 1.0;
                    let y = self.y_gen.next_sample(self.time);
                    points.push((x, y));
                }
            }
        }
        points
    }
}
