use crate::Renderer;

pub use iced_graphics::table::{Style, StyleSheet};

pub type Table<'a, Message> = iced_native::Table<'a, Message, Renderer>;