use iced::Alignment::Center;
use iced::Element;
use iced::widget::{button, center, column, row, text};

#[derive(Debug)]
pub struct CasePage {
    case: enlighten::Case,
    processing_state: ProcessingState,
}

#[derive(Debug, Copy, Clone)]
pub enum Message {
    CloseCase,
    CancelProcessing,
    AddEvidenceStore,
    Process,
}

pub enum NavigationMessage {
    CloseCase,
}

#[derive(Debug, Copy, Clone)]
enum ProcessingState {
    Processing,
    NotProcessing,
}

impl CasePage {
    pub fn new(case: enlighten::Case) -> Self {
        Self {
            case,
            processing_state: ProcessingState::NotProcessing,
        }
    }

    pub fn case_name(&self) -> String {
        self.case.case_name.clone()
    }

    pub fn view(&self) -> Element<'_, Message> {
        match self.processing_state {
            ProcessingState::Processing => self.processing_widget(),
            ProcessingState::NotProcessing => self.not_processing_widget(),
        }
    }

    pub fn update(&mut self, message: Message) -> Option<NavigationMessage> {
        match message {
            Message::AddEvidenceStore => None,
            Message::Process => {
                self.processing_state = ProcessingState::Processing;
                None
            }
            Message::CancelProcessing => {
                self.processing_state = ProcessingState::NotProcessing;
                None
            }
            Message::CloseCase => Some(NavigationMessage::CloseCase),
        }
    }

    fn processing_widget(&self) -> Element<'_, Message> {
        let title = text(format!("Processing: {}", &self.case.case_name));

        row![title, button("Cancel").on_press(Message::CancelProcessing)]
            .align_y(Center)
            .into()
    }

    fn not_processing_widget(&self) -> Element<'_, Message> {
        let title = text(format!("Hello from: {}", &self.case.case_name));

        column![
            title,
            button("Start processing").on_press(Message::Process),
            button("Close Case").on_press(Message::CloseCase),
        ]
        .align_x(Center)
        .into()
    }
}
