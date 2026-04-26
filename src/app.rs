use crate::cli::{Cli, Renderer};
use crate::framebuffer::FrameBuffer;
use crate::palette::Rgb;
use crate::render::TerminalBuffer;
use crate::signals::Oscilloscope;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::{
    cursor, queue,
    style::{Color, Print, SetForegroundColor},
};
use std::io::{Result, Write, stdout};
use std::time::{Duration, Instant};

#[derive(PartialEq)]
enum AppState {
    Running,
    Menu,
}

pub struct App {
    cli: Cli,
    width: u16,
    height: u16,
    should_quit: bool,
    framebuffer: FrameBuffer,
    term_buffer: TerminalBuffer,
    prev_term_buffer: TerminalBuffer,
    oscilloscope: Oscilloscope,
    state: AppState,
    fps: u32,
    frames_rendered: u32,
    last_fps_time: Instant,
    menu_selected: usize,
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
            state: AppState::Running,
            fps: 0,
            frames_rendered: 0,
            last_fps_time: Instant::now(),
            menu_selected: 0,
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
                        match self.state {
                            AppState::Running => match key.code {
                                KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
                                KeyCode::Char('m') => self.state = AppState::Menu,
                                KeyCode::Char('=') | KeyCode::Char('+') => {
                                    self.oscilloscope.scale_y *= 1.2
                                }
                                KeyCode::Char('-') => self.oscilloscope.scale_y /= 1.2,
                                KeyCode::Char(']') => self.oscilloscope.scale_x *= 1.2,
                                KeyCode::Char('[') => self.oscilloscope.scale_x /= 1.2,
                                _ => {}
                            },
                            AppState::Menu => match key.code {
                                KeyCode::Esc | KeyCode::Enter => {
                                    self.state = AppState::Running;
                                }
                                KeyCode::Up => {
                                    self.menu_selected = self.menu_selected.saturating_sub(1);
                                }
                                KeyCode::Down => {
                                    if self.menu_selected < 1 {
                                        self.menu_selected += 1;
                                    }
                                }
                                KeyCode::Left => {
                                    if self.menu_selected == 0 {
                                        self.cli.mode = self.cli.mode.prev();
                                        self.oscilloscope.mode = self.cli.mode;
                                    } else {
                                        self.cli.palette = self.cli.palette.prev();
                                    }
                                }
                                KeyCode::Right => {
                                    if self.menu_selected == 0 {
                                        self.cli.mode = self.cli.mode.next();
                                        self.oscilloscope.mode = self.cli.mode;
                                    } else {
                                        self.cli.palette = self.cli.palette.next();
                                    }
                                }
                                _ => {}
                            },
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
        self.frames_rendered += 1;
        if self.last_fps_time.elapsed() >= Duration::from_secs(1) {
            self.fps = self.frames_rendered;
            self.frames_rendered = 0;
            self.last_fps_time = Instant::now();
        }
        if self.state == AppState::Running {
            self.framebuffer.apply_decay(self.cli.decay);

            let points = self.oscilloscope.generate_chunk(150, 0.05);

            let margin = 0.8;

            let mut prev_point: Option<(f32, f32)> = None;

            for (math_x, math_y) in points {
                let (screen_x, screen_y) = self
                    .framebuffer
                    .map_coords(math_x * margin, math_y * margin);
                if let Some((px, py)) = prev_point {
                    self.framebuffer.draw_line(px, py, screen_x, screen_y, 1.0);
                }
                prev_point = Some((screen_x, screen_y));
            }
        }

        match self.cli.renderer {
            Renderer::Braille => self
                .term_buffer
                .render_braille(&self.framebuffer, &self.cli.palette),
            Renderer::Block => {}
        }

        let ui_color = self.cli.palette.ui_color();

        let info = format!(
            " MODE: {:?} | FPS: {} | V/DIV (Y): {:.2} | T/DIV (X): {:.2} ",
            self.cli.mode, self.fps, self.oscilloscope.scale_y, self.oscilloscope.scale_x
        );

        self.term_buffer.draw_text(0, 0, &info, ui_color);

        let hints = " [M] MENU  [+/-] SCALE Y  [[/]] SCALE X  [Q/ESC] QUIT ";
        self.term_buffer
            .draw_text(0, (self.height - 1) as usize, hints, ui_color);

        if self.state == AppState::Menu {
            let menu_w = 34;
            let menu_h = 8;
            let menu_x = (self.width as usize).saturating_sub(menu_w) / 2;
            let menu_y = (self.height as usize).saturating_sub(menu_h) / 2;

            self.term_buffer
                .draw_box(menu_x, menu_y, menu_w, menu_h, " SETTINGS ", ui_color);

            let sel_mode = if self.menu_selected == 0 { ">" } else { " " };
            let sel_pal = if self.menu_selected == 1 { ">" } else { " " };

            self.term_buffer.draw_text(
                menu_x + 2,
                menu_y + 2,
                &format!("{} MODE:    [{:?}]", sel_mode, self.cli.mode),
                ui_color,
            );
            self.term_buffer.draw_text(
                menu_x + 2,
                menu_y + 4,
                &format!("{} PALETTE: [{:?}]", sel_pal, self.cli.palette),
                ui_color,
            );

            self.term_buffer.draw_text(
                menu_x + 2,
                menu_y + 6,
                " [<-/->] CHANGE   [ESC] CLOSE ",
                Rgb(100, 100, 100),
            );
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
