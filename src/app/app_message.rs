use super::super::model::vide_type;
use crate::model::receive_msg::ReceiveMsg;

#[derive(Debug, Clone)]
pub enum Message {
    AudioPressed,
    //切换音频页面
    FileSelected,
    //选择文件
    LanguageSelected(vide_type::VideoContainerType),
    //接收并处理控制台消息
    ReceiveMsg(ReceiveMsg),
}