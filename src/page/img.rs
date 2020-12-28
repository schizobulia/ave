use iced::{{Column, Text}, Row, Align, Button, Container, Scrollable, Length, Command, Image, Slider};
use crate::app::app_message::Message;
use crate::style::{button_style, scrollable_style, container_style};
use crate::app::state::img::ImgState;
use crate::tool::file_tool::{get_filename};
use crate::tool::img_tool::{compression_img};
use std::path::PathBuf;
use crate::model::receive_msg::ReceiveMsg;


pub fn render(img_state: &mut ImgState) -> Column<Message> {
    let home_img = Image::new("resources/home.png");
    Column::new().spacing(15).push(
        Row::new().padding(5).spacing(10)
            .push(
                Button::new(&mut img_state.break_btn, home_img.width(Length::Units(20)).height(Length::Units(20)))
                    .on_press(Message::GoHome).style(button_style::Button::Light)
            )
    ).push(
        Column::new().padding(5).spacing(10).push(
            Text::new("请先选择图片,软件会自动开始压缩").size(18)
        ).push(
            Row::new().padding(5).align_items(Align::Center).push(
                Text::new("压缩质量").size(15)
            ).push(
                Slider::new(
                    &mut img_state.quality_progress,
                    10.0..=90.0,
                    img_state.quality_val,
                    Message::ImgQualityChanged,
                ).step(1.00)
            )
        ).push(
            Row::new().spacing(10).align_items(Align::Center)
                .push(
                    Button::new(&mut img_state.file_img_btn, Text::new("选择文件")).padding(5)
                        .style(button_style::Button::Primary)
                        .on_press(Message::FileSelected)
                ).push(
                Text::new(&img_state.create_img_path).size(18)
            )
        )
    ).push(
        Container::new(Scrollable::new(&mut img_state.scroll_comd_state)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(scrollable_style::Scrollable)
            .push(Text::new(
                &img_state.msg_conversion_statue,
            ).size(16))).height(Length::Units(500))
            .height(Length::Fill).style(container_style::Container::default())
    )
}


pub fn get_command(t_path: String, file_list: Vec<PathBuf>, quality: u8) -> Vec<Command<Message>> {
    let mut com_arr: Vec<Command<Message>> = Vec::new();
    let mut index = 1;
    for file in file_list {
        com_arr.push(Command::perform(compress_img(
            file, t_path.clone(), quality, index,
        ), Message::ReceiveMsg));
        index += 1;
    }
    com_arr
}


//压缩图片
async fn compress_img(file: PathBuf, t_path: String, quality: u8, index: i32) -> ReceiveMsg {
    let filename: String = file.to_string_lossy().to_string();
    let old_file_name = &get_filename(filename.clone());
    let tmp_type = "jpeg";
    let result = compression_img(
        format!("{}", file.to_string_lossy()),
        format!("{}//{}-{}.{}", t_path, old_file_name, index, tmp_type),
        quality,
    );

    let res: ReceiveMsg;
    if result {
        res = ReceiveMsg::new(filename.clone(), String::from("转换成功"));
    } else {
        res = ReceiveMsg::new(filename, String::from("转换失败"));
    }
    res
}