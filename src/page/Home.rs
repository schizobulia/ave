use iced::{Text, Button, Row, Column, PickList, Align};
use crate::app::app_message::Message;
use crate::style::button_style;
use crate::style::pick_list_style;
use crate::model::vide_type::VideoContainerType;

use nfd2::Response;
use std::thread;
use crate::tool::file_tool;
use crate::gstr;
use crate::app::state::home::HomeState;
use crate::tool::file_tool::now_dir_path;

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
    )
}

//转换视频格式
pub fn formatting_video(select_video_type: String) {
    match nfd2::dialog_multiple().open().expect("oh no") {
        Response::Okay(file_path) => {
            let _handle = thread::spawn(move || {
                let result = gstr::conversion::conversion_video(
                    format!("file:///{}", file_path.to_string_lossy()).as_str(),
                    file_tool::create_output_filename(&select_video_type,
                                                      &file_tool::get_filename(file_path.to_string_lossy().to_string())).as_str());
                if result.is_ok() {
                    file_tool::open_directory(now_dir_path().as_str());
                }
            });
        }
        Response::OkayMultiple(files) => {
            for file in  files {
                let tmp_type = select_video_type.clone();
                thread::spawn(move || {
                    let _res = gstr::conversion::conversion_video(
                        format!("file:///{}", file.to_string_lossy()).as_str(),
                        file_tool::create_output_filename(&tmp_type, &file_tool::get_filename(file.to_string_lossy().to_string())).as_str(),
                    );
                });
            }
        }
        Response::Cancel => println!("User canceled"),
    }
}