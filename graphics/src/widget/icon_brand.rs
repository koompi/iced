use crate::{backend, Backend, Primitive, Renderer};
pub use iced_native::icon_brand::{self, IconBrand, IconBrands};
use iced_native::{
    mouse, Color, Font, HorizontalAlignment, Rectangle, Size, VerticalAlignment,
};

impl<B> icon_brand::Renderer for Renderer<B>
where
    B: Backend + backend::Text,
{
    const ICONS_FONT: Font = Font::External {
        name: "Line Awesome",
        bytes: include_bytes!("../../fonts/la-brands-400.ttf"),
    };

    fn default_size(&self) -> u16 {
        self.backend().default_size()
    }

    fn measure(
        &self,
        content: IconBrands,
        size: u16,
        bounds: Size,
    ) -> (f32, f32) {
        self.backend().measure(
            &content.to_string(),
            f32::from(size),
            Self::ICONS_FONT,
            bounds,
        )
    }

    fn draw(
        &mut self,
        defaults: &Self::Defaults,
        bounds: Rectangle,
        _viewport: &Rectangle,
        content: IconBrands,
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
                font: Self::ICONS_FONT,
                horizontal_alignment,
                vertical_alignment,
            },
            mouse::Interaction::default(),
        )
    }
}
