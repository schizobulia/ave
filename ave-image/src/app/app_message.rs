use crate::model::image_type::ImageType;
use crate::model::receive_msg::ReceiveMsg;

#[derive(Debug, Clone)]
pub enum Message {
    FileSelected,
    //接收并处理控制台消息
    ReceiveMsg(ReceiveMsg),
    ImgQualityChanged(f32),
    ImgTypeSelected(ImageType),

    ResizeWidthChange(String),
    ResizeHeightChange(String),
}
