
// TODO:
//      - Simplify using instruction iterators
//      - Make movement mode
//      - Record modifiers

pub use collections::VecDeque;


mod editor;
pub use self::editor::*;

mod parse;
pub use self::parse::*;

mod position;
pub use self::position::*;

mod graphics;
pub use self::graphics::*;

mod mode;
pub use self::mode::*;

mod movement;
pub use self::movement::*;

mod cursor;
pub use self::cursor::*;

mod insert;
pub use self::insert::*;

mod delete;
pub use self::delete::*;

mod exec;
pub use self::exec::*;

pub fn main() {
    let mut editor = Editor::new();
}

