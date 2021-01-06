use iced::{button, pick_list, scrollable, slider};
use crate::model::vide_type::VideoContainerType;

//首页状态
pub struct HomeState {
    pub audio_page_btn: button::State,
    pub img_page_btn: button::State,
    pub file_home_btn: button::State,
    pub pick_list: pick_list::State<VideoContainerType>,
    pub select_video_type: VideoContainerType,
    pub create_video_path: String,
    pub scroll_comd_state: scrollable::State,
    pub msg_conversion_statue: String,
    pub quality_progress: slider::State,
    pub quality_val: f32, //视频压缩比例
}


impl Default for HomeState {
    fn default() -> Self {
        Self {
            audio_page_btn: button::State::default(),
            img_page_btn: button::State::default(),
            file_home_btn: button::State::default(),
            pick_list: pick_list::State::default(),
            select_video_type: VideoContainerType::Mp4,
            create_video_path: String::default(),
            scroll_comd_state: scrollable::State::new(),
            msg_conversion_statue: String::from("控制台...\r\n"),
            quality_val: 500.0,
            quality_progress: slider::State::default(),
        }
    }
}