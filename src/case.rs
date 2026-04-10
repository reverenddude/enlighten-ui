use iced::Alignment::Center;
use iced::Element;
use iced::Length;
use iced::widget::scrollable::{Direction, Scrollbar};
use iced::widget::{button, center, column, container, row, scrollable, table, text};

use crate::evidence_store_form::{self, EvidenceStoreForm};
use crate::processing_profile_form::{self, ProcessingProfileForm};
use enlighten::{
    Case, EnlightenField, FieldId, FieldValue, MetadataProfile, QueryBuilder, QueryResult,
};

#[derive(Debug)]
pub struct CasePage {
    case: Case,
    processing_state: CasePageState,
    current_metadata_profile: String,
    current_results: Vec<QueryResult>,
    current_search: QueryBuilder,
}

#[derive(Debug, Clone)]
pub enum Message {
    CloseCase,
    CancelProcessing,
    Process,

    NewProfile,
    EditProfile,
    ProfileForm(processing_profile_form::Message),

    NewEvidenceStore,
    EvidenceStoreForm(evidence_store_form::Message),
}

pub enum NavigationMessage {
    CloseCase,
}

#[derive(Debug, Clone)]
enum CasePageState {
    AddNewEvidence,
    Processing,
    DisplayResults,
    CreateEvidenceStore(EvidenceStoreForm),
    EditingProfile(ProcessingProfileForm),
}

impl CasePage {
    pub fn new(case: enlighten::Case) -> Self {
        let processing_state = match case.evidence_store.iter().count() > 0 {
            true => CasePageState::DisplayResults,
            false => CasePageState::AddNewEvidence,
        };

        let profile = case
            .get_all_metadata_profiles()
            .first()
            .unwrap_or(&&MetadataProfile::default())
            .name
            .clone();

        let search = QueryBuilder::new()
            //.select(case.get_metadata_profile(&profile).unwrap().fields.clone())
            .select(vec![
                EnlightenField::Guid.into(),
                EnlightenField::TopLevelGuid.into(),
                EnlightenField::MD5Hash.into(),
                EnlightenField::FileSize.into(),
                EnlightenField::IsTopLevel.into(),
                EnlightenField::Kind.into(),
                EnlightenField::MimeType.into(),
            ])
            .limit(50);
        let results = case.search(&search).unwrap();

        Self {
            case,
            processing_state,
            current_metadata_profile: profile,
            current_results: results,
            current_search: search,
        }
    }

    pub fn case_name(&self) -> String {
        self.case.case_name.clone()
    }

    pub fn view(&self) -> Element<'_, Message> {
        match self.processing_state {
            CasePageState::Processing => self.processing_widget(),
            CasePageState::DisplayResults => self.temp_results_thing(),
            CasePageState::AddNewEvidence => self.add_new_evidence_widget(),
            CasePageState::EditingProfile(ref processing_profile_form) => {
                processing_profile_form.view().map(Message::ProfileForm)
            }
            CasePageState::CreateEvidenceStore(ref evidence_store_form) => {
                evidence_store_form.view().map(Message::EvidenceStoreForm)
            }
        }
    }

    pub fn update(&mut self, message: Message) -> Option<NavigationMessage> {
        match message {
            Message::Process => {
                self.processing_state = CasePageState::Processing;
                let _ = self.case.process();
                self.current_results = self.case.search(&self.current_search).unwrap();
                self.processing_state = CasePageState::DisplayResults;
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
                        processing_profile_form::ProfileMessage::None => (),
                        processing_profile_form::ProfileMessage::Cancel => {
                            self.processing_state = CasePageState::AddNewEvidence;
                        }
                        processing_profile_form::ProfileMessage::ProcessingProfileSaved(
                            processing_profile,
                        ) => {
                            self.case.add_processing_profile(processing_profile);
                            self.processing_state = CasePageState::AddNewEvidence;
                        }
                    }
                }
                None
            }
            Message::EditProfile => {
                self.processing_state = CasePageState::EditingProfile(
                    ProcessingProfileForm::from_profile(&self.case.processing_profile),
                );
                None
            }
            Message::NewEvidenceStore => {
                self.processing_state =
                    CasePageState::CreateEvidenceStore(EvidenceStoreForm::new());
                None
            }
            Message::EvidenceStoreForm(evidence_msg) => {
                if let CasePageState::CreateEvidenceStore(store) = &mut self.processing_state {
                    match store.update(evidence_msg) {
                        evidence_store_form::EvidenceStoreMessage::None => (),
                        evidence_store_form::EvidenceStoreMessage::Cancel => {
                            self.processing_state = CasePageState::AddNewEvidence;
                        }
                        evidence_store_form::EvidenceStoreMessage::Create(evidence_store) => {
                            self.processing_state = CasePageState::AddNewEvidence;
                            let _ = self
                                .case
                                .add_evidence_store(evidence_store.name, evidence_store.store_path);
                        }
                    }
                }
                None
            }
        }
    }

    fn results_widget(&self) -> Element<'_, Message> {
        if self.current_results.len() == 0 {
            return text("No Results").into();
        }

        let columns = self
            .current_results
            .first()
            .unwrap()
            .fields
            .keys()
            .map(|field_id| {
                let field_id = field_id.clone();
                table::column(
                    text(field_id.to_string()), // header element
                    move |row: &QueryResult| {
                        // cell view fn
                        text(row.get(&field_id).to_string())
                    },
                )
            });

        table(columns, &self.current_results).into()
    }

    fn add_new_evidence_widget(&self) -> Element<'_, Message> {
        let content = column![
            self.processing_profile_widget(),
            self.create_evidence_store_widget(),
            row![
                button("Cancel").on_press(Message::CancelProcessing),
                button("Submit").on_press(Message::Process)
            ]
            .spacing(10)
        ];

        center(content).into()
    }

    fn create_evidence_store_widget(&self) -> Element<'_, Message> {
        row![container(row![
            button("Add Evidence Store").on_press(Message::NewEvidenceStore)
        ])]
        .into()
    }

    fn processing_profile_widget(&self) -> Element<'_, Message> {
        column![
            container(row![
                text(format!(
                    "Selected Profile: {}",
                    self.case.processing_profile.name.as_str()
                )),
                button("Edit").on_press(Message::EditProfile)
            ])
            .style(container::rounded_box)
        ]
        .into()
    }

    fn processing_widget(&self) -> Element<'_, Message> {
        let title = text(format!("Processing: {}", &self.case.case_name));

        row![title, button("Cancel").on_press(Message::CancelProcessing)]
            .align_y(Center)
            .into()
    }

    fn temp_results_thing(&self) -> Element<'_, Message> {
        column![
            button("Create Processing Profile").on_press(Message::NewProfile),
            button("Start processing").on_press(Message::Process),
            button("Close Case").on_press(Message::CloseCase),
            scrollable(self.results_widget()).direction(Direction::Both {
                horizontal: Scrollbar::default(),
                vertical: Scrollbar::default()
            }),
        ]
        .width(Length::Fill)
        .into()
    }
}
