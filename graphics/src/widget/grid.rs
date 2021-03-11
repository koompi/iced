use crate::{Backend, Primitive, Renderer};
use iced_native::grid;
use iced_native::mouse;
use iced_native::{Element, Layout, Point, Rectangle};

/// A container that distributes its contents vertically.
pub type Grid<'a, Message, Backend> =
    iced_native::Grid<'a, Message, Renderer<Backend>>;

impl<B> grid::Renderer for Renderer<B>
where
    B: Backend,
{
    const DEFAULT_PADDING: u16 = 0;
    const DEFAULT_SPACING: u16 = 8;

    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        children: &[Element<'_, Message, Self>],
    ) -> Self::Output {
        let mut mouse_interaction = mouse::Interaction::default();

        (
            Primitive::Group {
                primitives: children
                    .iter()
                    .zip(layout.children())
                    .map(|(child, layout)| {
                        let (primitive, new_mouse_interaction) = child.draw(
                            self,
                            defaults,
                            layout,
                            cursor_position,
                            viewport,
                        );

                        if new_mouse_interaction > mouse_interaction {
                            mouse_interaction = new_mouse_interaction;
                        }

                        primitive
                    })
                    .collect(),
            },
            mouse_interaction,
        )
    }
}
