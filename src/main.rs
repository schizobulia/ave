#![windows_subsystem = "windows"]
mod style;
mod app;
mod page;
mod gstr;
mod tool;
mod model;

use iced::{Sandbox, Element, Settings, window, Container, Text, Length };
use app::app_message::Message;
use crate::app::state::home::HomeState;

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
    home_page_state: HomeState
}

impl Sandbox for MainView {
    type Message = Message;

    fn new() -> MainView {
        MainView {
            page: String::from("home"),
            home_page_state: HomeState::default()
        }
    }

    fn title(&self) -> String {
        String::from("AVE")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::AudioPressed => {
                self.page = String::from("audio");
            }
            Message::FileSelected => {
                page::home::formatting_video(self.home_page_state.select_video_type.clone().to_string());
            }
            Message::LanguageSelected(vide_type) => {
                self.home_page_state.select_video_type = vide_type;
            }
        }
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