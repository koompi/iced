//! Stack multiple widgets overlay.
use crate::Renderer;
pub use iced_native::stack::Overflow;

/// A widget used to stack multiple widgets overlay.
///
/// This is an alias of an `iced_native` stack with a default
/// `Renderer`.
pub type Stack<'a, Message> = iced_native::Stack<'a, Message, Renderer>;