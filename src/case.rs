use iced::Alignment::Center;
use iced::Element;
use iced::widget::{button, center, column, container, pick_list, row, text, text_input};

use enlighten::{EvidenceStore, ProcessingProfile};

use crate::processing_profile_form::{self, ProcessingProfileForm};

#[derive(Debug)]
pub struct CasePage {
    case: enlighten::Case,
    processing_state: CasePageState,
}

#[derive(Debug, Clone)]
pub enum Message {
    CloseCase,
    CancelProcessing,
    Process,

    NewProfile,
    ProfileForm(processing_profile_form::Message),
    SaveProfile,
}

#[derive(Debug, Clone)]
pub struct NewEvidenceStoreSettings {
    store_name: String,
    store_path: String,
}

#[derive(Debug, Clone)]
pub struct NewProcessingProfileState {
    name: String,
}

pub enum NavigationMessage {
    CloseCase,
}

#[derive(Debug, Clone)]
enum CasePageState {
    AddNewEvidence,
    Processing,
    DisplayResults,
    EditingProfile(ProcessingProfileForm),
}

impl CasePage {
    pub fn new(case: enlighten::Case) -> Self {
        let processing_state = match case.evidence_store.iter().count() > 0 {
            true => CasePageState::DisplayResults,
            false => CasePageState::AddNewEvidence,
        };

        Self {
            case,
            processing_state,
        }
    }

    pub fn case_name(&self) -> String {
        self.case.case_name.clone()
    }

    pub fn view(&self) -> Element<'_, Message> {
        match self.processing_state {
            CasePageState::Processing => self.processing_widget(),
            CasePageState::DisplayResults => self.results_widget(),
            CasePageState::AddNewEvidence => self.add_new_evidence_widget(),
            CasePageState::EditingProfile(ref processing_profile_form) => {
                processing_profile_form.view().map(Message::ProfileForm)
            }
        }
    }

    pub fn update(&mut self, message: Message) -> Option<NavigationMessage> {
        match message {
            Message::Process => {
                self.processing_state = CasePageState::Processing;
                None
            }
            Message::CancelProcessing => {
                self.processing_state = CasePageState::DisplayResults;
                None
            }
            Message::CloseCase => Some(NavigationMessage::CloseCase),
            Message::NewProfile => {
                self.processing_state = CasePageState::EditingProfile(ProcessingProfileForm::new());
                None
            }
            Message::ProfileForm(profile_msg) => {
                if let CasePageState::EditingProfile(profile) = &mut self.processing_state {
                    match profile.update(profile_msg) {
                        processing_profile_form::ProfileMessage::None => return None,
                        processing_profile_form::ProfileMessage::ProcessingProfileSaved(
                            processing_profile,
                        ) => {
                            self.case.processing_profile = processing_profile;
                            self.processing_state = CasePageState::AddNewEvidence;
                            return None;
                        }
                    }
                }
                None
            }
            Message::SaveProfile => todo!(),
        }
    }

    fn add_new_evidence_widget(&self) -> Element<'_, Message> {
        let content = column![
            self.new_processing_profile_widget(),
            row![
                button("Cancel").on_press(Message::CancelProcessing),
                button("Submit").on_press(Message::Process)
            ]
            .spacing(10)
        ];

        center(content).into()
    }

    fn processing_profile_selector_widget(&self) -> Element<'_, Message> {
        todo!()
    }

    fn new_processing_profile_widget(&self) -> Element<'_, Message> {
        column![container(row![]).style(container::rounded_box)].into()
    }

    fn processing_widget(&self) -> Element<'_, Message> {
        let title = text(format!("Processing: {}", &self.case.case_name));

        row![title, button("Cancel").on_press(Message::CancelProcessing)]
            .align_y(Center)
            .into()
    }

    fn results_widget(&self) -> Element<'_, Message> {
        let content = column![
            button("Create Processing Profile").on_press(Message::NewProfile),
            button("Start processing").on_press(Message::Process),
            button("Close Case").on_press(Message::CloseCase),
        ]
        .align_x(Center);

        center(content).into()
    }
}
