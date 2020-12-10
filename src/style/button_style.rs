use iced::{button, Background, Color, Vector};

//reference：https://v4.bootcss.com/docs/components/buttons/
pub enum Button {
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
    Light,
    Dark
}

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(match self {
                Button::Primary => Color::from_rgb8(1, 123, 255),
                Button::Secondary => Color::from_rgb8(108, 117, 125),
                Button::Success => Color::from_rgb8(40, 167, 68),
                Button::Danger => Color::from_rgb8(220, 53, 69),
                Button::Warning => Color::from_rgb8(255, 193, 7),
                Button::Info => Color::from_rgb8(23, 162, 184),
                Button::Light => Color::from_rgb8(248, 249, 250),
                Button::Dark => Color::from_rgb8(52, 58, 64),
            })),
            border_radius: 12 as f32,
            shadow_offset: Vector::new(1.0, 1.0),
            text_color: Color::WHITE,
            ..button::Style::default()
        }
    }
}