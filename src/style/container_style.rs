use iced::{container, Color, Background};

//容器样式
pub struct Container {
    pub(crate) background: Color,
    pub(crate) text_color: Color,
}

impl Default for Container {
    fn default() -> Self {
        Container {
            background: Color::from_rgb8(
                0x36, 0x39, 0x3F,
            ),
            text_color: Color::WHITE,
        }
    }
}

impl container::StyleSheet for Container {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(self.background)),
            text_color: Some(Color::WHITE),
            border_color: Color::WHITE,
            ..container::Style::default()
        }
    }
}