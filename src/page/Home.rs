use iced::{Text, Button, Row, Column};
use crate::app::app_message::Message;
use crate::style::button_style;
use iced::button::State;

//首页
pub fn render<'a>(audio: &'a mut State, file_btn: &'a mut State) -> Column<'a, Message> {
    Column::new().spacing(80).push(
        Row::new().push(
            Button::new(audio, Text::new("音频处理")).padding(5)
                .style(button_style::Button::Primary)
                .on_press(Message::AudioPressed)
        )
    ).push(
        Column::new().padding(100).push(
            Button::new(file_btn, Text::new("选择文件")).padding(5)
                .style(button_style::Button::Primary)
                .on_press(Message::FileSelected)
        )
    )
}