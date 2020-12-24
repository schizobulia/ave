use iced::{Align, Command};
use iced::PickList;
use iced::Column;
use iced::Row;
use iced::Button;
use iced::Text;
use iced::Scrollable;
use crate::app::app_message::Message;
use crate::style::button_style;
use crate::style::pick_list_style;
use crate::model::vide_type::VideoContainerType;
use crate::tool::file_tool;
use crate::gstr;
use crate::app::state::home::HomeState;
use crate::tool::file_tool::{get_file_list};
use std::path::PathBuf;

//首页
pub fn render(home_state: &mut HomeState) -> Column<Message> {
    let pick_list = PickList::new(
        &mut home_state.pick_list,
        &VideoContainerType::ALL[..],
        Some(home_state.select_video_type),
        Message::LanguageSelected,
    ).style(pick_list_style::PickList);
    Column::new().spacing(20).push(
        Row::new().push(
            Button::new(&mut home_state.audio_page_btn, Text::new("音频处理(目前正在开发中)")).padding(5)
                .style(button_style::Button::Info)
                .on_press(Message::AudioPressed)
        )
    ).push(
        Column::new().padding(10).spacing(10)
            .push(
                Text::new("请先选择需要最终转换的格式,然后选择文件,\
                软件会自动开始转换,成功之后将自动打开文件夹").size(18)
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
        Scrollable::new(&mut home_state.scroll_comd_state)
            .padding(10)
            .max_width(500)
            .max_height(350)
            .push(Text::new(
                &home_state.msg_conversion_statue,
            ))
    )
}


pub fn get_command(select_type: String) -> Vec<Command<Message>> {
    let file_list = get_file_list();
    let mut com_arr: Vec<Command<Message>> = Vec::new();
    for file in file_list {
        let tmp_type = select_type.clone();
        com_arr.push(Command::perform(formatting_video(
            tmp_type, file), Message::ReceiveMsg));
    }
    com_arr
}

//转换视频格式
async fn formatting_video(tmp_type: String, file: PathBuf) -> String {
    let old_file_name = &file_tool::get_filename(file.to_string_lossy().to_string());
    let result = gstr::conversion::conversion_video(
        format!("file:///{}", file.to_string_lossy()).as_str(),
        file_tool::create_output_filename(tmp_type.as_str(), &old_file_name).as_str(),
    );
    let mut res = String::from(old_file_name);
    if result.is_ok() {
        res.push_str("转换成功...");
    } else {
        res.push_str("转换失败");
    }
    res
}