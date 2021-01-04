use iced::{{Column, Text}, Row, Align, Button, Container, Scrollable, Length, Command, Slider, PickList};
use crate::app::app_message::Message;
use iced_style::{button_style, scrollable_style, container_style, pick_list_style};
use crate::app::state::img::ImgState;
use ave_tool::file_tool::get_filename;
use ave_tool::img_tool::compression_img;
use std::path::PathBuf;
use crate::model::receive_msg::ReceiveMsg;
use crate::model::image_type::ImageType;


pub fn render(img_state: &mut ImgState) -> Column<Message> {
    let pick_list = PickList::new(
        &mut img_state.pick_list,
        &ImageType::ALL[..],
        Some(img_state.select_img_type),
        Message::ImgTypeSelected,
    ).style(pick_list_style::PickList);

    Column::new().spacing(15).push(
        Column::new().padding(5).spacing(10).push(
            Text::new("请先选择需要最终转换的格式,软件会自动开始压缩").size(18)
        ).push(
            Row::new().padding(3).align_items(Align::Center).push(
                Text::new("压缩质量：").size(15)
            ).push(
                Slider::new(
                    &mut img_state.quality_progress,
                    10.0..=90.0,
                    img_state.quality_val,
                    Message::ImgQualityChanged,
                ).step(1.00)
            )
        ).push(
            Row::new().padding(3).align_items(Align::Center)
                .push(Text::new("生成格式：").size(15))
                .push(pick_list)
        ).push(
            Row::new().padding(3).align_items(Align::Center)
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


pub fn get_command(t_path: String, file_list: Vec<PathBuf>, quality: u8, img_type: String) -> Vec<Command<Message>> {
    let mut com_arr: Vec<Command<Message>> = Vec::new();
    let mut index = 1;
    for file in file_list {
        com_arr.push(Command::perform(compress_img(
            file, t_path.clone(), quality, index, img_type.clone(),
        ), Message::ReceiveMsg));
        index += 1;
    }
    com_arr
}


//压缩图片
async fn compress_img(file: PathBuf, t_path: String, quality: u8, index: i32, img_type: String) -> ReceiveMsg {
    let filename: String = file.to_string_lossy().to_string();
    let old_file_name = &get_filename(filename.clone());
    let result = compression_img(
        format!("{}", file.to_string_lossy()),
        format!("{}//{}-{}.{}", t_path, old_file_name, index, img_type),
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