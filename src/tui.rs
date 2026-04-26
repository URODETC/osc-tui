use crossterm::{
    cursor, execute, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{stdout, Result};
use std::panic;

pub fn setup() -> Result<()> {
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, cursor::Hide)?;
    Ok(())
}

pub fn restore() -> Result<()> {
    let mut stdout = stdout();
    execute!(stdout, cursor::Show, LeaveAlternateScreen)?;
    Ok(())
}

pub fn install_panic_hook() {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        let _ = restore();
        original_hook(panic_info);
    }))
}