use crate::cli::{Cli, Renderer};
use crate::framebuffer::FrameBuffer;
use crate::render::TerminalBuffer;
use crate::signals::Oscilloscope;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::{
    cursor, queue,
    style::{Color, Print, SetForegroundColor},
};
use std::io::{Result, Write, stdout};
use std::time::{Duration, Instant};

pub struct App {
    cli: Cli,
    width: u16,
    height: u16,
    should_quit: bool,
    framebuffer: FrameBuffer,
    term_buffer: TerminalBuffer,
    prev_term_buffer: TerminalBuffer,
    oscilloscope: Oscilloscope,
}

impl App {
    pub fn new(cli: Cli) -> Result<Self> {
        let (width, height) = crossterm::terminal::size()?;

        let (sub_w, sub_h) = match cli.renderer {
            Renderer::Braille => (width as usize * 2, height as usize * 4),
            Renderer::Block => (width as usize, height as usize * 2),
        };

        Ok(Self {
            oscilloscope: Oscilloscope::new(cli.mode),
            cli,
            width,
            height,
            should_quit: false,
            framebuffer: FrameBuffer::new(sub_w, sub_h),
            term_buffer: TerminalBuffer::new(width as usize, height as usize),
            prev_term_buffer: TerminalBuffer::new(width as usize, height as usize),
        })
    }

    pub fn run(&mut self) -> Result<()> {
        let frame_duration = Duration::from_secs_f32(1.0 / 60.0);
        while !self.should_quit {
            let frame_start = Instant::now();
            self.handle_events()?;
            self.update();
            self.draw()?;

            let elapsed = frame_start.elapsed();
            if elapsed < frame_duration {
                std::thread::sleep(frame_duration - elapsed);
            }
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(16))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') => self.should_quit = true,
                            _ => {}
                        }
                    }
                }
                Event::Resize(cols, rows) => {
                    self.width = cols;
                    self.height = rows;
                    let (sub_w, sub_h) = match self.cli.renderer {
                        Renderer::Braille => (cols as usize * 2, rows as usize * 4),
                        Renderer::Block => (cols as usize, rows as usize * 2),
                    };
                    self.framebuffer.resize(sub_w, sub_h);
                    self.term_buffer = TerminalBuffer::new(cols as usize, rows as usize);

                    self.prev_term_buffer = TerminalBuffer::new(cols as usize, rows as usize);
                    let mut out = stdout();
                    queue!(
                        out,
                        crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
                    )?;
                    out.flush()?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn update(&mut self) {
        self.framebuffer.apply_decay(self.cli.decay);

        let points = self.oscilloscope.generate_chunk(150, 0.05);

        let mut prev_point: Option<(f32, f32)> = None;

        for (math_x, math_y) in points {
            let (screen_x, screen_y) = self.framebuffer.map_coords(math_x, math_y);
            if let Some((px, py)) = prev_point {
                self.framebuffer.draw_line(px, py, screen_x, screen_y, 1.0);
            }
            prev_point = Some((screen_x, screen_y));
        }

        match self.cli.renderer {
            Renderer::Braille => self
                .term_buffer
                .render_braille(&self.framebuffer, &self.cli.palette),
            Renderer::Block => {}
        }
    }

    fn draw(&mut self) -> Result<()> {
        let mut out = stdout();
        let mut changed = false;

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = (y as usize) * (self.width as usize) + (x as usize);
                let current = self.term_buffer.cells[idx];
                let prev = self.prev_term_buffer.cells[idx];

                if current != prev {
                    queue!(
                        out,
                        cursor::MoveTo(x, y),
                        SetForegroundColor(Color::Rgb {
                            r: current.fg.0,
                            g: current.fg.1,
                            b: current.fg.2
                        }),
                        Print(current.ch)
                    )?;
                    self.prev_term_buffer.cells[idx] = current;
                    changed = true;
                }
            }
        }

        if changed {
            out.flush()?;
        }
        Ok(())
    }
}
