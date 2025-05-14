use std::io::stdout;

use crossterm::event::{read, Event::Key, KeyCode::Char};
use crossterm::event::{KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Editor { should_quit: false }
    }

    pub fn run(&mut self) {
        if let Err(err) = self.repl() {
            panic!("{err:#?}");
        }
        println!("GoodBye!");
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        enable_raw_mode().expect("Failed to enable raw mode");
        loop {
            if let Key(KeyEvent {
                code,
                modifiers,
                kind,
                state,
            }) = read()?
            {
                println!(
                    "Code: {code:?} Modifiers: {modifiers:?} Kind: {kind:?} State: {state:?} \r"
                );
                match code {
                    Char('q') if modifiers == KeyModifiers::CONTROL => {
                        self.should_quit = true;
                    }
                    _ => (),
                }
            }
            if self.should_quit {
                break;
            }
        }
        disable_raw_mode().expect("Failed to disable raw mode");
        Ok(())
    }

    fn clear_screen() -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))
    }
}
