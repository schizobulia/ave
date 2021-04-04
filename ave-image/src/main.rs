#![windows_subsystem = "windows"]

mod app;
mod model;
mod page;

use crate::app::state::img::ImgState;
use crate::model::image_type::ImageType;
use app::app_message::Message;
use ave_tool::datetime::now_time;
use ave_tool::file_tool::{get_file_list, mkdir, now_dir_path};
use iced::{executor, window, Application, Clipboard, Command, Container, Element, Settings, Text};

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
    img_page_state: ImgState,
}

impl Application for MainView {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (MainView, Command<Self::Message>) {
        (
            MainView {
                page: String::from("img"),
                img_page_state: ImgState::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("AVE  (项目处于开发阶段,我的邮箱：2833324528@qq.com)")
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        match message {
            Message::FileSelected => match self.page.as_str() {
                "img" => {
                    let file_list = get_file_list(ImageType::default().get_all_type().as_str());
                    if file_list.len() > 0 {
                        let dir = mkdir(format!("{}\\out\\img\\{}", now_dir_path(), now_time()));
                        self.img_page_state.create_img_path = format!("图片生成目录：{}", dir);
                        let com_arr =
                            page::img::get_command(dir, file_list, &mut self.img_page_state);
                        return Command::batch(com_arr);
                    }
                }
                _ => {}
            },

            Message::ReceiveMsg(msg) => match self.page.as_str() {
                "img" => {
                    let old_msg = &self.img_page_state.msg_conversion_statue;
                    self.img_page_state.msg_conversion_statue = format!(
                        "{}{}\r\n\
                    ",
                        old_msg,
                        msg.to_string()
                    );
                }
                _ => {}
            },

            Message::ImgTypeSelected(img_type) => self.img_page_state.select_img_type = img_type,
            Message::ImgQualityChanged(q) => self.img_page_state.quality_val = q,
            Message::ResizeWidthChange(width) => match width.parse::<u32>() {
                Ok(s) => {
                    self.img_page_state.resize_width = s.to_string();
                }
                Err(_) => {
                    self.img_page_state.resize_width = String::from("");
                }
            },
            Message::ResizeHeightChange(height) => match height.parse::<u32>() {
                Ok(s) => {
                    self.img_page_state.resize_height = s.to_string();
                }
                Err(_) => {
                    self.img_page_state.resize_height = String::from("");
                }
            },
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        match self.page.as_str() {
            "img" => Container::new(page::img::render(&mut self.img_page_state)).into(),
            _ => Container::new(Text::new("页面异常")).into(),
        }
    }
}

fn main() {
    application();
}
