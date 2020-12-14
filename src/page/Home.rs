use iced::{Text, Button, Row, Column, PickList, pick_list};
use crate::app::app_message::Message;
use crate::style::button_style;
use iced::button::State;
use crate::model::vide_type::VideoContainerType;

//首页
pub fn render<'a>(audio: &'a mut State, file_btn: &'a mut State,
                  list_def: &'a mut pick_list::State<VideoContainerType>,
                  select_video_type: &'a mut VideoContainerType
) -> Column<'a, Message> {
    let pick_list = PickList::new(
        list_def,
        &VideoContainerType::ALL[..],
        Some(*select_video_type),
        Message::LanguageSelected,
    );
    Column::new().spacing(20).push(
        Row::new().push(
            Button::new(audio, Text::new("音频处理")).padding(5)
                .style(button_style::Button::Primary)
                .on_press(Message::AudioPressed)
        )
    ).push(
        Column::new().padding(10).spacing(10)
            .push(
                Text::new("请先选择需要最终转换的格式,然后选择文件,软件会自动开始转换(成功之后将自动打开文件夹)")
            )
            .push(
                Row::new().spacing(10)
                    .push(
                        pick_list
                    )
                    .push(
                        Button::new(file_btn, Text::new("选择文件")).padding(5)
                            .style(button_style::Button::Primary)
                            .on_press(Message::FileSelected)
                    )
            )
    )
}