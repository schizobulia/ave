use iced::{Text, Button, Row, Column, PickList, pick_list};
use crate::app::app_message::Message;
use crate::style::button_style;
use iced::button::State;
use crate::model::vide_type::VideoContainerType;


//首页
pub fn render<'a>(audio: &'a mut State, file_btn: &'a mut State, list_def: &'a mut pick_list::State<VideoContainerType>) -> Column<'a, Message> {
    let pick_list = PickList::new(
        list_def,
        &VideoContainerType::ALL[..],
        Some(VideoContainerType::Mp4),
        Message::LanguageSelected,
    );

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
        ).push(
            pick_list
        )
    )
}