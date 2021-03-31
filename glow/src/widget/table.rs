use crate::Renderer;

pub use iced_graphics::table::{Style, StyleSheet};
pub use iced_native::table::{State, TableData, TableError, TableOptions, TableOrder, TableResult};

pub type Table<'a, Message> = iced_native::Table<'a, Message, Renderer>;
