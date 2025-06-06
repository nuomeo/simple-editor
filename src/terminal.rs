use std::io::{stdout, Write};

use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{queue, Command};

pub struct Terminal {}

#[derive(Copy, Clone)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}
#[derive(Copy, Clone, Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Terminal {
    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position { x: 0, y: 0 })?;
        Self::execute()?;
        Ok(())
    }

    pub fn terminate() -> Result<(), std::io::Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn show_cursor() -> Result<(), std::io::Error> {
        Self::queue_command(Show)?;
        Ok(())
    }

    pub fn hide_cursor() -> Result<(), std::io::Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), std::io::Error> {
        Self::queue_command(Clear(ClearType::All))
    }

    pub fn clear_line() -> Result<(), std::io::Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))
    }

    pub fn print(msg: &str) -> Result<(), std::io::Error> {
        Self::queue_command(Print(msg))?;
        Ok(())
    }

    pub fn move_cursor_to(pos: Position) -> Result<(), std::io::Error> {
        Self::queue_command(MoveTo(pos.x as u16, pos.y as u16))?;
        Ok(())
    }

    pub fn size() -> Result<Size, std::io::Error> {
        let (width, height) = size()?;
        Ok(Size { height: height.into(), width: width.into()})
    }

    pub fn execute() -> Result<(), std::io::Error> {
        stdout().flush()?;
        Ok(())
    }

    fn queue_command<T: Command>(command: T) -> Result<(), std::io::Error> {
        queue!(stdout(), command)?;
        Ok(())
    }
}
