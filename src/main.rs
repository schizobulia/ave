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
use crate::tool::file_tool::now_dir_path;

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
}

impl Application for MainView {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (MainView, Command<Self::Message>) {
        (MainView {
            page: String::from("home"),
            home_page_state: HomeState::default(),
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
            Message::FileSelected => {
                page::home::formatting_video(
                    &mut self.home_page_state);
                self.home_page_state.create_video_path = format!("生成视频目录：{}", &*now_dir_path());
            }

            Message::LanguageSelected(vide_type) => {
                self.home_page_state.select_video_type = vide_type;
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