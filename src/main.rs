#![feature(stmt_expr_attributes)]

#[cfg(feature = "orbital")]
extern crate orbital;

#[macro_use]
pub mod debug;

pub mod editor;
pub mod buffer;
pub mod parse;
pub mod key_state;
pub mod key;
pub mod prompt;
pub mod open;
pub mod redraw;
pub mod options;
pub mod position;
pub mod graphics;
pub mod selection;
pub mod mode;
pub mod movement;
pub mod motion;
pub mod cursor;
pub mod insert;
pub mod delete;
pub mod exec;
pub mod invert;

fn main() {
    editor::Editor::init();
}
