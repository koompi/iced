use crate::{Backend, Primitive, Renderer};
use iced_native::stack::{self, Overflow};
use iced_native::{mouse, Element, Layout, Point, Rectangle};

/// A widget used to stack multiple widgets overlay.
pub type Stack<'a, Message, Backend> =
    iced_native::Stack<'a, Message, Renderer<Backend>>;

impl<B> stack::Renderer for Renderer<B>
where B: Backend
{
    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        _overflow: &Overflow,
        children: &[(Element<'_, Message, Self>, Option<Point>)],
    ) -> Self::Output {
        let mut mouse_interaction = mouse::Interaction::default();

        (
            Primitive::Group {
                primitives: children
                .iter()
                .zip(layout.children())
                .map(|((element, _), layout)| {
                    let (primitive, new_mouse_interaction) = element.draw(self, defaults, layout, cursor_position, viewport);

                    if new_mouse_interaction > mouse_interaction {
                        mouse_interaction = new_mouse_interaction;
                    }

                    primitive
                })
                .collect(),
            },
            mouse_interaction
        )
    }
}