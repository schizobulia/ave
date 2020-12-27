// #![windows_subsystem = "windows"]

mod style;
mod app;
mod page;
mod gstr;
mod tool;
mod model;

use iced::{Application, executor, Element, Settings, window, Container, Text, Length, Command};
use app::app_message::Message;
use crate::app::state::home::HomeState;
use crate::tool::file_tool::{now_dir_path, mkdir, get_file_list};
use crate::tool::datetime::now_time;
use crate::app::state::img::ImgState;
use crate::model::vide_type::VideoContainerType;
use crate::model::image_type::ImageType;

fn application() {
    let _result = MainView::run(Settings {
        window: window::Settings {
            size: (800, 500),
            min_size: None,
            max_size: None,
            resizable: true,
            decorations: true,
            transparent: false,
            always_on_top: false,
            icon: None,
        },
        //此处请参考：https://github.com/hecrj/iced/issues/403#issuecomment-674344559
        default_font: Some(include_bytes!("../fonts/Dengl.ttf")),
        ..Settings::default()
    });
}

#[derive(Default)]
struct MainView {
    page: String,
    home_page_state: HomeState,
    img_page_state: ImgState,
}

impl Application for MainView {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (MainView, Command<Self::Message>) {
        (MainView {
            page: String::from("home"),
            home_page_state: HomeState::default(),
            img_page_state: ImgState::default(),
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("AVE  (项目处于开发阶段,我的邮箱：2833324528@qq.com)")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::AudioPressed => {
                // self.page = String::from("audio");
            }
            Message::ImgPressed => {
                self.page = String::from("img");
            }
            Message::FileSelected => {
                match self.page.as_str() {
                    "home" => {
                        let file_list = get_file_list(VideoContainerType::default().get_all_type().as_str());
                        if file_list.len() > 0 {
                            let dir = mkdir(format!("{}\\out\\video\\{}", now_dir_path(), now_time()));
                            self.home_page_state.create_video_path = format!("视频生成目录：{}", dir);
                            let com_arr = page::home::get_command(
                                self.home_page_state.select_video_type.to_string(),
                                dir, file_list);
                            return Command::batch(com_arr);
                        }
                    }
                    "img" => {
                        let file_list = get_file_list(ImageType::default().get_all_type().as_str());
                        if file_list.len() > 0 {
                            let dir = mkdir(format!("{}\\out\\img\\{}", now_dir_path(), now_time()));
                            self.img_page_state.create_img_path = format!("图片生成目录：{}", dir);
                            let com_arr = page::img::get_command(
                                dir, file_list);
                            return Command::batch(com_arr);
                        }
                    }
                    _ => {}
                }
            }

            Message::LanguageSelected(vide_type) => {
                self.home_page_state.select_video_type = vide_type;
            }

            Message::ReceiveMsg(msg) => {
                match self.page.as_str() {
                    "home" => {
                        let old_msg = &self.home_page_state.msg_conversion_statue;
                        self.home_page_state.msg_conversion_statue =
                            format!("{}{}\r\n\
                    ", old_msg, msg.to_string());
                    }
                    "img" => {
                        let old_msg = &self.img_page_state.msg_conversion_statue;
                        self.img_page_state.msg_conversion_statue =
                            format!("{}{}\r\n\
                    ", old_msg, msg.to_string());
                    }
                    _ => {}
                }
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        match self.page.as_str() {
            "home" => {
                Container::new(page::home::render(&mut self.home_page_state))
                    .height(Length::Fill)
                    .width(Length::Fill)
                    .into()
            }
            "audio" => {
                Container::new(page::audio::render())
                    .into()
            }
            "img" => {
                Container::new(page::img::render(&mut self.img_page_state))
                    .into()
            }
            _ => {
                Container::new(Text::new("页面异常"))
                    .into()
            }
        }
    }
}

fn main() {
    application();
}