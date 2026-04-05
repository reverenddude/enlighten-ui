use iced::widget::{button, column, row, text};
use iced::{Alignment, Center, Element};

#[derive(Default, Debug, Clone, Copy)]
pub(crate) struct Home;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Loaded,
    NewCase,
    OpenCase,
}

impl Home {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self) -> Element<'_, Message> {
        row![
            column![
                text("Hello from Home"),
                button("Create New Case").on_press(Message::NewCase),
                button("Open Existing Case").on_press(Message::OpenCase)
            ]
            .max_width(500)
            .align_x(Center)
        ]
        .align_y(Center)
        .into()
    }
}
