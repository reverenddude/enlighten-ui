use std::path::PathBuf;

use iced::Alignment::Center;
use iced::Element;
use iced::widget::{button, column, container, row, text, text_input};

use enlighten::{EvidenceStore, ProcessingProfile};

#[derive(Debug)]
pub struct CasePage {
    case: enlighten::Case,
    processing_state: CasePageState,
    new_evidence_state: NewEvidenceState,
}

#[derive(Debug)]
struct NewEvidenceState {
    source_path: String,
}

#[derive(Debug, Copy, Clone)]
pub enum Message {
    CloseCase,
    CancelProcessing,
    AddEvidenceStore,
    CreateProcessingProfile,
    Process,
}

pub enum NavigationMessage {
    CloseCase,
}

#[derive(Debug, Copy, Clone)]
enum CasePageState {
    CreatingNewEvidence,
    Processing,
    NotProcessing,
}

impl CasePage {
    pub fn new(case: enlighten::Case) -> Self {
        Self {
            case,
            processing_state: CasePageState::NotProcessing,
            new_evidence_state: NewEvidenceState {
                source_path: String::new(),
            },
        }
    }

    pub fn case_name(&self) -> String {
        self.case.case_name.clone()
    }

    pub fn view(&self) -> Element<'_, Message> {
        match self.processing_state {
            CasePageState::Processing => self.processing_widget(),
            CasePageState::NotProcessing => self.not_processing_widget(),
            CasePageState::CreatingNewEvidence => self.add_new_evidence_widget(),
        }
    }

    pub fn update(&mut self, message: Message) -> Option<NavigationMessage> {
        match message {
            Message::AddEvidenceStore => None,
            Message::Process => {
                self.processing_state = CasePageState::Processing;
                None
            }
            Message::CancelProcessing => {
                self.processing_state = CasePageState::NotProcessing;
                None
            }
            Message::CloseCase => Some(NavigationMessage::CloseCase),
            Message::CreateProcessingProfile => todo!(),
        }
    }

    fn add_new_evidence_widget(&self) -> Element<'_, Message> {
        column![
            container(row![
                text("Evidence Store Name"),
                text_input("Source Path..", &self.new_evidence_state.source_path)
            ])
            .style(container::rounded_box)
        ]
        .into()
    }

    fn new_processing_profile_widget(&self) -> Element<'_, Message> {
        todo!()
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
