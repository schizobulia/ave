#![windows_subsystem = "windows"]
mod style;
mod app;
mod page;
mod gstr;

use iced::{Sandbox, Element, Settings, window, Container, button, Text, Length};
use app::app_message::Message;
use nfd2::Response;

fn application() {
    MainView::run(Settings {
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
    audio_page_btn: button::State,
    file_home_btn: button::State,
}

impl Sandbox for MainView {
    type Message = Message;

    fn new() -> MainView {
        MainView {
            page: String::from("home"),
            audio_page_btn: button::State::default(),
            file_home_btn: button::State::default(),
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
                match nfd2::open_file_dialog(None, None).expect("oh no") {
                    Response::Okay(file_path) => {
                        gstr::conversion::conversion_video(&*file_path.to_string_lossy(), "");
                    }
                    Response::OkayMultiple(files) => println!("Files {:?}", files),
                    Response::Cancel => println!("User canceled"),
                }
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        match self.page.as_str() {
            "home" => {
                Container::new(page::home::render(&mut self.audio_page_btn, &mut self.file_home_btn))
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