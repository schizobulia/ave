use super::super::model::vide_type;

#[derive(Debug, Clone)]
pub enum Message {
    AudioPressed,   //切换音频页面
    FileSelected,  //选择文件
    LanguageSelected(vide_type::VideoContainerType),
    ReceiveMsg(String)
}