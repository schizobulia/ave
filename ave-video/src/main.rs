#![windows_subsystem = "windows"]

mod app;
mod gstr;
mod model;
mod page;

use crate::app::state::home::HomeState;
use crate::model::vide_type::VideoContainerType;
use app::app_message::Message;
use ave_tool::datetime::now_time;
use ave_tool::file_tool::{get_file_list, mkdir, now_dir_path};
use iced::{executor, window, Application, Command, Container, Element, Length, Settings, Text, Clipboard};

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
        default_font: Some(include_bytes!("../../fonts/Dengl.ttf")),
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
        (
            MainView {
                page: String::from("home"),
                home_page_state: HomeState::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("AVE  (项目处于开发阶段,我的邮箱：2833324528@qq.com)")
    }

    fn update(&mut self, message: Self::Message, _clipboard: &mut Clipboard) -> Command<Self::Message> {
        match message {
            Message::FileSelected => match self.page.as_str() {
                "home" => {
                    let file_list =
                        get_file_list(VideoContainerType::default().get_all_type().as_str());
                    if file_list.len() > 0 {
                        let dir = mkdir(format!("{}\\out\\video\\{}", now_dir_path(), now_time()));
                        self.home_page_state.create_video_path = format!("视频生成目录：{}", dir);
                        let com_arr =
                            page::home::get_command(&mut self.home_page_state, dir, file_list);
                        return Command::batch(com_arr);
                    }
                }
                _ => {}
            },

            Message::ReceiveMsg(msg) => match self.page.as_str() {
                "home" => {
                    let old_msg = &self.home_page_state.msg_conversion_statue;
                    self.home_page_state.msg_conversion_statue = format!(
                        "{}{}\r\n\
                    ",
                        old_msg,
                        msg.to_string()
                    );
                }
                _ => {}
            },
            Message::VideoQualityChanged(val) => self.home_page_state.quality_val = val,
            Message::LanguageSelected(vide_type) => {
                self.home_page_state.select_video_type = vide_type
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        match self.page.as_str() {
            "home" => Container::new(page::home::render(&mut self.home_page_state))
                .height(Length::Fill)
                .width(Length::Fill)
                .into(),
            _ => Container::new(Text::new("页面异常")).into(),
        }
    }
}

fn main() {
    application();
}
