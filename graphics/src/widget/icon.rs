use crate::{backend, Backend, Primitive, Renderer};
use iced_native::{
    mouse, Font, HorizontalAlignment, Color, Rectangle, VerticalAlignment, Size
};
pub use iced_native::icon::{self, Icon, Icons};


impl<B> icon::Renderer for Renderer<B>
where
    B: Backend + backend::Text,
{
    const ICON_FONT: Font = Font::External {
        name: "Line Awesome",
        bytes: include_bytes!("../../fonts/la-solid-900.ttf"),
    };

    fn default_size(&self) -> u16 {
        self.backend().default_size()
    }

    fn measure(&self, content: Icons, size: u16, bounds: Size) -> (f32, f32) {
        self.backend()
            .measure(&content.to_string(), f32::from(size), B::ICON_FONT, bounds)
    }

    fn draw(
        &mut self,
        defaults: &Self::Defaults,
        bounds: Rectangle,
        _viewport: &Rectangle,
        content: Icons,
        size: u16,
        color: Option<Color>,
        _label: &str,
        horizontal_alignment: HorizontalAlignment,
        vertical_alignment: VerticalAlignment,
    ) -> Self::Output {
        let x = match horizontal_alignment {
            iced_native::HorizontalAlignment::Left => bounds.x,
            iced_native::HorizontalAlignment::Center => bounds.center_x(),
            iced_native::HorizontalAlignment::Right => bounds.x + bounds.width,
        };

        let y = match vertical_alignment {
            iced_native::VerticalAlignment::Top => bounds.y,
            iced_native::VerticalAlignment::Center => bounds.center_y(),
            iced_native::VerticalAlignment::Bottom => bounds.y + bounds.height,
        };

        (
            Primitive::Text {
                content: content.to_string(),
                size: f32::from(size),
                bounds: Rectangle { x, y, ..bounds },
                color: color.unwrap_or(defaults.text.color),
                font: B::ICON_FONT,
                horizontal_alignment,
                vertical_alignment,
            },
            mouse::Interaction::default(),
        )
    }
}