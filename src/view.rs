use crate::{buffer::Buffer, terminal::{Position, Size, Terminal}};

pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct View {
    buffer: Buffer,
}

impl View {
    pub fn render(&self) -> Result<(), std::io::Error> {
        let Size { height, .. } = Terminal::size()?;

        for current_row in 0..height {
            Terminal::clear_line()?;
            if current_row < self.buffer.size() {
                Terminal::print(self.buffer.get(current_row).as_str())?;
            } else {
                Self::draw_empty_rows()?;
            }
            
            if self.buffer.size() == 0 && current_row == height / 3 {
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