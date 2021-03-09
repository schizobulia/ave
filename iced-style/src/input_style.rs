use iced::{text_input, Background, Color};

//输入默认样式
#[allow(dead_code)]
pub struct TextInput;

impl text_input::StyleSheet for TextInput {
    fn active(&self) -> text_input::Style {
        text_input::Style {
            background: Background::Color(Color::WHITE),
            border_radius: 4 as f32,
            border_width: 1.0,
            border_color: Color {
                a: 0.7,
                ..Color::from_rgb8(206, 212, 218)
            },
        }
    }

    fn focused(&self) -> text_input::Style {
        text_input::Style {
            border_color: Color {
                a: 0.6,
                ..Color::from_rgb8(184, 218, 255)
            },
            ..Self.active()
        }
    }

    fn placeholder_color(&self) -> Color {
        Color::from_rgb(0.7, 0.7, 0.7)
    }

    fn value_color(&self) -> Color {
        Color::BLACK
    }

    fn selection_color(&self) -> Color {
        Color::TRANSPARENT
    }
}
