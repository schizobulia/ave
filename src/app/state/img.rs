use iced::{button, scrollable, slider};

//首页状态
pub struct ImgState {
    pub img_page_btn: button::State,
    pub file_img_btn: button::State,
    pub break_btn: button::State,
    pub create_img_path: String,
    pub scroll_comd_state: scrollable::State,
    pub msg_conversion_statue: String,
    pub quality_progress: slider::State,
    pub quality_val: f32,
}

impl Default for ImgState {
    fn default() -> Self {
        Self {
            img_page_btn: button::State::default(),
            file_img_btn: button::State::default(),
            break_btn: button::State::default(),
            create_img_path: String::default(),
            scroll_comd_state: scrollable::State::new(),
            msg_conversion_statue: String::from("控制台...\r\n"),
            quality_progress: slider::State::default(),
            quality_val: 75.0,
        }
    }
}