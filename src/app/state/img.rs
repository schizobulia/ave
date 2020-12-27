use iced::{button, scrollable};

//首页状态
pub struct ImgState {
    pub img_page_btn: button::State,
    pub file_img_btn: button::State,
    pub create_img_path: String,
    pub scroll_comd_state: scrollable::State,
    pub msg_conversion_statue: String,
}

impl Default for ImgState {
    fn default() -> Self {
        Self {
            img_page_btn: button::State::default(),
            file_img_btn: button::State::default(),
            create_img_path: String::default(),
            scroll_comd_state: scrollable::State::new(),
            msg_conversion_statue: String::from("控制台...\r\n"),
        }
    }
}