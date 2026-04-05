use iced::Alignment::Center;
use iced::Element;
use iced::widget::{button, center, column, row, text};

pub(crate) struct Processing;

struct ProcessingState;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    DisplayCase,
}

impl Processing {
    pub fn view(&self) -> Element<'_, Message> {
        row![
            text("Hello from processing"),
            button("Go to existing Case").on_press(Message::DisplayCase)
        ]
        .align_y(Center)
        .into()
    }
}
