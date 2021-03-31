use crate::Renderer;

pub use iced_graphics::table::{Style, StyleSheet};
pub use iced_native::table::{State, TableData, TableError, TableOptions, TableOrder, TableResult};

/// A widget that produces a message when clicked.
///
/// This is an alias of an `iced_native` button with an `iced_wgpu::Renderer`.
pub type Table<'a, Message> = iced_native::Table<'a, Message, Renderer>;
