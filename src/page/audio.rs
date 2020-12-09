use iced::{{Column, Text}};
use crate::app::app_message::Message;


pub fn render<'a>() -> Column<'a, Message> {
    Column::new().push(
        Text::new("音频页面")
    )
}
