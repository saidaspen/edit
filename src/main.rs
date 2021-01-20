#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod document;
mod terminal;
mod row;

use editor::Editor;
use editor::Position;
pub use terminal::Terminal;
pub use document::Document;
pub use row::Row;

fn main() {
    Editor::default().run();
}
