use iced::Alignment::Center;
use iced::Element;
use iced::widget::{button, center, column, row, text};

#[derive(Debug)]
pub struct CasePage {
    case: enlighten::Case,
}

#[derive(Debug, Copy, Clone)]
pub enum Message {
    CloseCase,
    AddEvidenceStore,
    Process,
}

pub enum NavigationMessage {
    CloseCase,
}

impl CasePage {
    pub fn new(case: enlighten::Case) -> Self {
        Self { case }
    }
    pub fn view(&self) -> Element<'_, Message> {
        let title = text(format!("Hello from: {}", &self.case.case_name));

        row![title, button("Close Case").on_press(Message::CloseCase)]
            .align_y(Center)
            .into()
    }

    pub fn update(&mut self, message: Message) -> Option<NavigationMessage> {
        match message {
            Message::AddEvidenceStore => {
                todo!()
            }
            Message::Process => {
                todo!()
            }
            Message::CloseCase => return Some(NavigationMessage::CloseCase),
        };
    }
}
