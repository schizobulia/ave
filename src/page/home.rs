use iced::{Align, Command, Container, Length, PickList, Column, Row, Button, Text, Scrollable};
use crate::app::app_message::Message;
use crate::style::{button_style, pick_list_style, scrollable_style, container_style};
use crate::model::vide_type::VideoContainerType;
use crate::gstr;
use crate::app::state::home::HomeState;
use crate::tool::file_tool::{get_filename};
use std::path::PathBuf;
use crate::model::receive_msg::ReceiveMsg;

//首页
pub fn render(home_state: &mut HomeState) -> Column<Message> {
    let pick_list = PickList::new(
        &mut home_state.pick_list,
        &VideoContainerType::ALL[..],
        Some(home_state.select_video_type),
        Message::LanguageSelected,
    ).style(pick_list_style::PickList);
    Column::new().spacing(15).push(
        Row::new().padding(5).spacing(10)
            .push(
                Button::new(&mut home_state.img_page_btn, Text::new("图片处理"))
                    .style(button_style::Button::Primary)
                    .on_press(Message::ImgPressed)
            ).push(
            Button::new(&mut home_state.audio_page_btn, Text::new("音频处理(目前正在开发中)")).padding(5)
                .style(button_style::Button::Info)
                .on_press(Message::AudioPressed)
        )
    ).push(
        Column::new().padding(5).spacing(10)
            .push(
                Text::new("请先选择需要最终转换的格式,然后选择文件,\
                软件会自动开始转换").size(18)
            )
            .push(
                Row::new().spacing(10).align_items(Align::Center)
                    .push(
                        pick_list
                    )
                    .push(
                        Button::new(&mut home_state.file_home_btn, Text::new("选择文件")).padding(5)
                            .style(button_style::Button::Primary)
                            .on_press(Message::FileSelected)
                    ).push(
                    Text::new(&home_state.create_video_path).size(18)
                )
            )
    ).push(
        Container::new(Scrollable::new(&mut home_state.scroll_comd_state)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(scrollable_style::Scrollable)
            .push(Text::new(
                &home_state.msg_conversion_statue,
            ).size(16))).height(Length::Units(500))
            .height(Length::Fill).style(container_style::Container::default())
    )
}


pub fn get_command(select_type: String, t_path: String, file_list: Vec<PathBuf>) -> Vec<Command<Message>> {
    let mut com_arr: Vec<Command<Message>> = Vec::new();
    let mut index = 1;
    for file in file_list {
        let tmp_type = select_type.clone();
        com_arr.push(Command::perform(formatting_video(
            tmp_type.to_string(), file, t_path.clone(), index), Message::ReceiveMsg));
        index += 1;
    }
    com_arr
}

//转换视频格式
async fn formatting_video(tmp_type: String, file: PathBuf, t_path: String, index: i32) -> ReceiveMsg {
    let filename: String = file.to_string_lossy().to_string();
    let old_file_name = &get_filename(filename.clone());
    let result = gstr::conversion::conversion_video(
        format!("file:///{}", file.to_string_lossy()).as_str(),
        format!("{}//{}-{}.{}", t_path, old_file_name, index, tmp_type).as_str(),
    );
    let res: ReceiveMsg;
    if result.is_ok() {
        res = ReceiveMsg::new(filename, String::from("转换成功"));
    } else {
        res = ReceiveMsg::new(filename, String::from("转换失败"));
    }
    res
}