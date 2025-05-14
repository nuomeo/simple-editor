mod editor;
mod terminal;
mod view;
mod buffer;
use editor::{Editor};

fn main() {
    Editor::default().run();
}
