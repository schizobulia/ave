use crate::app::app_message::Message;
use crate::app::state::img::ImgState;
use crate::model::image_type::ImageType;
use crate::model::receive_msg::ReceiveMsg;
use ave_tool::file_tool::get_filename;
use ave_tool::img_tool::{get_dynamic_image, quality_img, set_dynamic_image_resize};
use iced::{
    Align, Button, Command, Container, HorizontalAlignment, Length, PickList, Row, Scrollable,
    Slider, TextInput, VerticalAlignment, {Column, Text},
};
use iced_style::{button_style, container_style, input_style, pick_list_style, scrollable_style};
use std::fs;
use std::path::PathBuf;

pub fn render(img_state: &mut ImgState) -> Column<Message> {
    let pick_list = PickList::new(
        &mut img_state.pick_list,
        &ImageType::ALL[..],
        Some(img_state.select_img_type),
        Message::ImgTypeSelected,
    )
    .style(pick_list_style::PickList);

    Column::new()
        .spacing(15)
        .push(
            Column::new()
                .padding(5)
                .spacing(10)
                .push(Text::new("请先选择需要最终转换的格式,软件会自动开始压缩").size(18))
                .push(
                    Row::new()
                        .padding(3)
                        .align_items(Align::Center)
                        .push(Text::new("压缩质量：").size(15))
                        .push(
                            Slider::new(
                                &mut img_state.quality_progress,
                                10.0..=90.0,
                                img_state.quality_val,
                                Message::ImgQualityChanged,
                            )
                            .step(1.00),
                        ),
                )
                .push(
                    Row::new()
                        .padding(3)
                        .align_items(Align::Center)
                        .push(Text::new("生成格式：").size(15))
                        .push(pick_list),
                )
                .push(
                    Row::new()
                        .padding(3)
                        .align_items(Align::Center)
                        .push(Text::new("图片大小：").size(15))
                        .push(
                            TextInput::new(
                                &mut img_state.resize_width_state,
                                "宽度",
                                img_state.resize_width.as_str(),
                                Message::ResizeWidthChange,
                            )
                            .padding(5)
                            .width(Length::Units(100))
                            .style(input_style::TextInput),
                        )
                        .push(
                            Text::new("*")
                                .width(Length::Units(20))
                                .horizontal_alignment(HorizontalAlignment::Center),
                        )
                        .push(
                            TextInput::new(
                                &mut img_state.resize_heigth_state,
                                "高度",
                                img_state.resize_height.as_str(),
                                Message::ResizeHeightChange,
                            )
                            .padding(5)
                            .width(Length::Units(100))
                            .style(input_style::TextInput),
                        )
                        .push(
                            Text::new("   默认不填写,则不处理图片宽高。")
                                .size(13)
                                .vertical_alignment(VerticalAlignment::Bottom),
                        ),
                )
                .push(
                    Row::new()
                        .padding(3)
                        .align_items(Align::Center)
                        .push(
                            Button::new(&mut img_state.file_img_btn, Text::new("选择文件"))
                                .padding(5)
                                .style(button_style::Button::Primary)
                                .on_press(Message::FileSelected),
                        )
                        .push(Text::new(&img_state.create_img_path).size(18)),
                ),
        )
        .push(
            Container::new(
                Scrollable::new(&mut img_state.scroll_comd_state)
                    .padding(10)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .style(scrollable_style::Scrollable)
                    .push(Text::new(&img_state.msg_conversion_statue).size(16)),
            )
            .height(Length::Units(500))
            .height(Length::Fill)
            .style(container_style::Container::default()),
        )
}

pub fn get_command(
    t_path: String,
    file_list: Vec<PathBuf>,
    img_state: &mut ImgState,
) -> Vec<Command<Message>> {
    let mut com_arr: Vec<Command<Message>> = Vec::new();
    let quality = img_state.quality_val as u8;
    let img_type = img_state.select_img_type.to_string();
    let wdith = &img_state.resize_width;
    let height = &img_state.resize_height;

    let mut index = 1;
    for file in file_list {
        com_arr.push(Command::perform(
            dispose_img(
                file,
                t_path.clone(),
                quality,
                index,
                img_type.clone(),
                wdith.to_string(),
                height.to_string(),
            ),
            Message::ReceiveMsg,
        ));
        index += 1;
    }
    com_arr
}

//处理图片
async fn dispose_img(
    file: PathBuf,
    t_path: String,
    quality: u8,
    index: i32,
    img_type: String,
    width: String,
    height: String,
) -> ReceiveMsg {
    let filename: String = file.to_string_lossy().to_string();
    let old_file_name = &get_filename(filename.clone());
    let mut dynamic_image = get_dynamic_image(file.to_string_lossy().to_string());

    if !width.is_empty() && !height.is_empty() {
        dynamic_image = set_dynamic_image_resize(
            dynamic_image,
            width.parse::<u32>().unwrap(),
            height.parse::<u32>().unwrap(),
        );
    }

    let f = fs::File::create(format!(
        "{}//{}-{}.{}",
        t_path, old_file_name, index, img_type
    ))
    .unwrap();
    let result = quality_img(dynamic_image, f, quality);
    let res: ReceiveMsg;
    if result {
        res = ReceiveMsg::new(filename.clone(), String::from("转换成功"), index);
    } else {
        res = ReceiveMsg::new(filename, String::from("转换失败"), index);
    }
    res
}
