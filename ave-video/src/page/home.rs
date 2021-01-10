use iced::{Align, Command, Container, Length, PickList, Column, Row, Button, Text, Scrollable, Slider};
use crate::app::app_message::Message;
use iced_style::{button_style, pick_list_style, scrollable_style, container_style};
use crate::model::vide_type::VideoContainerType;
use crate::gstr;
use crate::app::state::home::HomeState;
use ave_tool::file_tool::{get_filename, mkdir};
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
        Column::new().padding(5).spacing(10)
            .push(
                Text::new("请先选择需要最终转换的格式,然后选择文件,\
                软件会自动开始转换").size(18)
            )
            .push(
                Row::new().padding(3).align_items(Align::Center).push(
                    Text::new("压缩质量：").size(15)
                ).push(
                    Slider::new(
                        &mut home_state.quality_progress,
                        10.0..=1000.0,
                        home_state.quality_val,
                        Message::VideoQualityChanged,
                    ).step(1.00)
                )
            )
            .push(
                Row::new().padding(3).align_items(Align::Center)
                    .push(Text::new("生成格式：").size(15))
                    .push(pick_list)
            )
            .push(
                Row::new().spacing(10).align_items(Align::Center)
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


pub fn get_command(home_state: &mut HomeState, t_path: String, file_list: Vec<PathBuf>) -> Vec<Command<Message>> {
    let select_type = home_state.select_video_type;
    let quality_val = home_state.quality_val;
    let mut com_arr: Vec<Command<Message>> = Vec::new();
    let mut index = 1;
    for file in file_list {
        let tmp_type = select_type.clone();
        let filename = file.file_name().unwrap().to_string_lossy();
        let create_path = get_create_path(t_path.clone(), home_state.select_video_type, filename.to_string());
        com_arr.push(Command::perform(formatting_video(
            tmp_type, file.clone(), create_path, index, quality_val), Message::ReceiveMsg));
        index += 1;

        let old_msg = &home_state.msg_conversion_statue;
        home_state.msg_conversion_statue = format!("{}{}   转换中...\r\n\
                    ", old_msg, filename);
    }
    com_arr
}

//转换视频格式
async fn formatting_video(tmp_type: VideoContainerType, file: PathBuf, t_path: String, index: i32, quality_val: f32) -> ReceiveMsg {
    let filename: String = file.to_string_lossy().to_string();
    let old_file_name = &get_filename(filename.clone());
    let result = gstr::conversion::conversion_video(
        file.to_string_lossy().to_string().as_str(),
        format!("{}//{}-{}.{}", t_path, old_file_name, index, tmp_type.to_string()).as_str(),
        quality_val as i32,
        tmp_type,
    );
    let res: ReceiveMsg;
    if result.is_ok() {
        res = ReceiveMsg::new(filename, String::from("转换成功"));
    } else {
        res = ReceiveMsg::new(filename, String::from("转换失败"));
    }
    res
}


//对特殊视频格式做处理
//生成m3u8视频时  单独给每个视频创建文件夹
fn get_create_path(t_path: String, video_type: VideoContainerType, filename: String) -> String {
    match video_type {
        VideoContainerType::M3u8 => {
            let mut m3u8_dir = t_path.clone();
            m3u8_dir.push_str("/");
            m3u8_dir.push_str(filename.as_str());
            mkdir(m3u8_dir)
        }
        _ => {
            t_path
        }
    }
}