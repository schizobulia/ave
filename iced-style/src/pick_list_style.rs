use iced::{pick_list, Color};

pub struct PickList;

//参考：https://www.litefeel.com/tools/ascii.php
const ACTIVE: Color = Color::from_rgb(
    0xFF as f32 / 255.0,
    0xFF as f32 / 255.0,
    0xFF as f32 / 255.0,
);

impl pick_list::StyleSheet for PickList {
    fn menu(&self) -> pick_list::Menu {
        pick_list::Menu {
            text_color: Color::from_rgb8(73, 80, 87),
            background: ACTIVE.into(),
            border_width: 1.0,
            border_color: Color {
                a: 0.7,
                ..Color::from_rgb8(206, 212, 218)
            },
            selected_background: Color {
                a: 0.5,
                ..Color::BLACK
            }
            .into(),
            selected_text_color: Color::from_rgb8(73, 80, 87),
        }
    }

    fn active(&self) -> pick_list::Style {
        pick_list::Style {
            text_color: Color::from_rgb8(73, 80, 87),
            background: ACTIVE.into(),
            border_width: 1.0,
            border_color: Color {
                a: 0.6,
                ..Color::from_rgb8(184, 218, 255)
            },
            border_radius: 2.0,
            icon_size: 0.5,
        }
    }

    fn hovered(&self) -> pick_list::Style {
        let active = self.active();
        pick_list::Style {
            border_width: 2.0,
            border_color: Color {
                a: 0.9,
                ..Color::from_rgb8(184, 218, 255)
            },
            ..active
        }
    }
}
