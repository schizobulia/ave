use super::super::model::vide_type;
use crate::model::receive_msg::ReceiveMsg;

#[derive(Debug, Clone)]
pub enum Message {
    //切换音频页面
    AudioPressed,
    FileSelected,
    //选择文件
    LanguageSelected(vide_type::VideoContainerType),
    //接收并处理控制台消息
    ReceiveMsg(ReceiveMsg),
    //切换图片处理页面
    ImgPressed,
    ImgQualityChanged(f32),
    //返回上一页
    GoHome,
}