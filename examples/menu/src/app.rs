use iced::{
    button, Align, Button, Column, Container, Element, IconBrand, IconBrands,
    Text,
};

#[derive(Debug, Clone)]
pub struct App {
    pub icon: IconBrands,
    pub name: String,
    pub clickable: button::State,
}

#[derive(Debug, Clone, Copy)]
pub enum AppMessage {
    AppClicked,
}

impl App {
    pub fn new(icon: IconBrands, name: String) -> Self {
        Self {
            icon,
            name,
            clickable: button::State::new(),
        }
    }

    pub fn update(&mut self, message: AppMessage) {
        match message {
            AppMessage::AppClicked => {}
        }
    }

    pub fn view(&mut self) -> Element<AppMessage> {
        let icon = IconBrand::new(self.icon).size(127);
        let name = Text::new(&self.name);
        let app = Column::new()
            .align_items(Align::Center)
            .push(icon)
            .push(name);
        let container = Container::new(app);
        let app_btn = Button::new(&mut self.clickable, container)
            .padding(10)
            .on_press(AppMessage::AppClicked)
            .style(style::CustomButton);
        Container::new(app_btn).center_x().center_y().into()
    }
}

mod style {
    use iced::{button, Color};
    pub struct CustomButton;

    impl button::StyleSheet for CustomButton {
        fn active(&self) -> button::Style {
            button::Style {
                text_color: Color::BLACK,
                background: Some(Color::TRANSPARENT.into()),
                border_radius: 12.0,
                ..button::Style::default()
            }
        }

        fn hovered(&self) -> button::Style {
            let active = self.active();

            button::Style {
                background: Some(
                    Color {
                        a: 0.2,
                        ..Color::BLACK
                    }
                    .into(),
                ),
                ..active
            }
        }
    }
}
