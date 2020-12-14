use iced::{button, pick_list };
use crate::model::vide_type::VideoContainerType;

//首页状态
pub struct HomeState {
    pub audio_page_btn: button::State,
    pub file_home_btn: button::State,
    pub pick_list: pick_list::State<VideoContainerType>,
    pub select_video_type: VideoContainerType,
    pub create_video_path: String,
}


impl Default for HomeState {
    fn default() -> Self {
        Self {
            audio_page_btn: button::State::default(),
            file_home_btn: button::State::default(),
            pick_list: pick_list::State::default(),
            select_video_type: VideoContainerType::Mp4,
            create_video_path: String::default(),
        }
    }
}