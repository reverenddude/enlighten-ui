use iced::Alignment::Center;
use iced::Element;
use iced::widget::{button, center, column, row, text};

#[derive(Debug)]
pub struct Case;

#[derive(Debug, Copy, Clone)]
pub enum Message {
    CloseCase,
}

impl Case {
    pub fn view(&self, case: &enlighten::Case) -> Element<'_, Message> {
        let title = text(format!("Hello from: {}", case.case_name.clone()));

        row![title, button("Close Case").on_press(Message::CloseCase)]
            .align_y(Center)
            .into()
    }
}
