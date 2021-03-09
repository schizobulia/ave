use iced::{scrollable, Background, Color};

pub struct Scrollable;

//控制台样式
impl scrollable::StyleSheet for Scrollable {
    fn active(&self) -> scrollable::Scrollbar {
        scrollable::Scrollbar {
            background: Some(Background::Color(Color::from_rgb8(0x36, 0x36, 0x3F))),
            border_radius: 2.0,
            border_width: 0.0,
            border_color: Color::WHITE,
            scroller: scrollable::Scroller {
                color: Color::from_rgb8(88, 86, 86),
                border_radius: 2.0,
                border_width: 0.0,
                border_color: Color::WHITE,
            },
        }
    }

    fn hovered(&self) -> scrollable::Scrollbar {
        let active = self.active();

        scrollable::Scrollbar { ..active }
    }
}
