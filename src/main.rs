// #![windows_subsystem = "windows"]
mod style;
mod app;
mod page;
mod gstr;
mod tool;
mod model;

use iced::{Sandbox, Element, Settings, window, Container, button, Text, Length, pick_list, };
use app::app_message::Message;
use nfd2::Response;
use std::thread;
use tool::datetime;
use tool::file_tool;
use model::vide_type::VideoContainerType;
use std::sync::mpsc;

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
    audio_page_btn: button::State,
    file_home_btn: button::State,
    pick_list: pick_list::State<VideoContainerType>,
    select_video_type: VideoContainerType,
    video_status: String,

}

impl Sandbox for MainView {
    type Message = Message;

    fn new() -> MainView {
        MainView {
            page: String::from("home"),
            audio_page_btn: button::State::default(),
            file_home_btn: button::State::default(),
            pick_list: pick_list::State::default(),
            select_video_type: VideoContainerType::Mp4,
            video_status: String::from("未选择需要转换的文件"),
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
                        let (tx, rx) = mpsc::channel();
                        self.video_status = String::from("转换中...");

                        let _handle = thread::spawn(move || {
                            let received: String = rx.recv().unwrap();
                            let result = gstr::conversion::conversion_video(
                                &*file_path.to_string_lossy(), datetime::create_output_filename(received.as_str()).as_str());
                            if result.is_ok() {
                                file_tool::open_directory("c:\\");
                            }
                        });
                        let _tx_send = tx.send(self.select_video_type.to_string());
                    }
                    Response::OkayMultiple(files) => println!("Files {:?}", files),
                    Response::Cancel => println!("User canceled"),
                }
            }
            Message::LanguageSelected(vide_type) => {
                self.select_video_type = vide_type;
            }
        }
    }



    fn view(&mut self) -> Element<Message> {
        match self.page.as_str() {
            "home" => {
                Container::new(page::home::render(
                    &mut self.audio_page_btn, &mut self.file_home_btn,
                    &mut self.pick_list, &mut self.select_video_type,
                    &mut self.video_status))
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