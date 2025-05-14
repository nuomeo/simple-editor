use std::cmp::min;

use crossterm::event::{read, Event::Key, KeyCode::*};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::terminal::{Position, Size, Terminal};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default, Clone, Copy)]
pub struct Location {
    x: usize,
    y: usize,
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    location: Location,
}

impl Editor {
    pub fn default() -> Self {
        Editor {
            should_quit: false,
            location: Location { x: 0, y: 0 },
        }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }

            let event = read()?;
            self.evaluate_event(&event)?;
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor()?;
        Terminal::move_cursor_to(Position::default())?;
        if self.should_quit {
            Terminal::clear_screen()?;
            print!("GoodBye! \r\n");
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(Position {
                x: self.location.x,
                y: self.location.y,
            })?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    fn move_location(&mut self, code: KeyCode) -> Result<(), std::io::Error> {
        let Location { mut x, mut y } = self.location;
        let Size { width, height } = Terminal::size()?;
        match code {
            Left => {
                x = x.saturating_sub(1);
            }
            Right => {
                x = min(x.saturating_add(1), width.saturating_sub(1));
            }
            Up => {
                y = y.saturating_sub(1);
            }
            Down => {
                y = min(y.saturating_add(1), height.saturating_sub(1));
            }
            Home => {
                x = 0;
            }
            End => {
                x = width.saturating_sub(1);
            }
            PageDown => {
                y = height.saturating_sub(1);
            }
            PageUp => {
                y = 0;
            }
            _ => (),
        }
        self.location = Location { x, y };
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) -> Result<(), std::io::Error> {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                Left | Right | Up | Down | Home | End | PageDown | PageUp => {
                    self.move_location(*code)?;
                }
                _ => (),
            }
        }
        Ok(())
    }

    pub fn draw_rows() -> Result<(), std::io::Error> {
        let Size { height, .. } = Terminal::size()?;
        for current_row in 0..height {
            Terminal::clear_line()?;

            Self::draw_empty_rows()?;
            if current_row == height / 3 {
                Self::draw_welcome_msg()?;
            }

            if current_row + 1 < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    pub fn draw_empty_rows() -> Result<(), std::io::Error> {
        Terminal::print("~")?;
        Ok(())
    }

    pub fn draw_welcome_msg() -> Result<(), std::io::Error> {
        let Size { height, width } = Terminal::size()?;
        let welcome_message = format!("{} -- version {}", NAME, VERSION);
        let welcome_message_len = welcome_message.len();
        let padding = (width - welcome_message_len) / 2;
        Terminal::move_cursor_to(Position {
            x: padding,
            y: height / 3,
        })?;
        Terminal::print(&welcome_message)?;
        Ok(())
    }
}
