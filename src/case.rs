use iced::Alignment::Center;
use iced::Element;
use iced::Length;
use iced::widget::checkbox;
use iced::widget::{button, center, column, container, mouse_area, row, scrollable, table, text};
use std::collections::HashSet;
use uuid::Uuid;

use crate::evidence_store_form::{self, EvidenceStoreForm};
use crate::processing_profile_form::{self, ProcessingProfileForm};
use enlighten::{Case, MetadataProfile, QueryBuilder, QueryResult};

#[derive(Debug)]
pub struct CasePage {
    case: Case,
    processing_state: CasePageState,
    current_metadata_profile: String,
    current_results: Vec<QueryResult>,
    current_search: QueryBuilder,
    selected_ids: HashSet<Uuid>,
    select_all_records: bool,
    selected_id: Option<Uuid>,
}

#[derive(Debug, Clone)]
pub enum Message {
    CloseCase,
    CancelProcessing,
    Process,

    AddNewEvidence,

    NewProfile,
    EditProfile,
    ProfileForm(processing_profile_form::Message),

    NewEvidenceStore,
    EvidenceStoreForm(evidence_store_form::Message),

    RecordToggled(Uuid, bool),
    SelectAllRecordsToggled(bool),
    RecordSelected(Uuid),
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
            .to_owned()
            .clone();

        let search = QueryBuilder::new()
            //.select(case.get_metadata_profile(&profile).unwrap().fields.clone())
            .select(profile.fields)
            .limit(50);
        let results = case.search(&search).unwrap();

        Self {
            case,
            processing_state,
            current_metadata_profile: profile.name,
            current_results: results,
            current_search: search,
            selected_ids: HashSet::new(),
            select_all_records: false,
            selected_id: None,
        }
    }

    pub fn case_name(&self) -> String {
        self.case.case_name.clone()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let tool_bar = self.tool_bar_widget();
        let bottom_bar = self.bootom_bar_widget();

        let content = match self.processing_state {
            CasePageState::Processing => self.processing_widget(),
            CasePageState::DisplayResults => match self.case.evidence_store.iter().count() {
                0 => self.add_new_evidence_widget(),
                _ => self.results_widget(),
            },
            CasePageState::AddNewEvidence => self.add_new_evidence_widget(),
            CasePageState::EditingProfile(ref processing_profile_form) => {
                processing_profile_form.view().map(Message::ProfileForm)
            }
            CasePageState::CreateEvidenceStore(ref evidence_store_form) => {
                evidence_store_form.view().map(Message::EvidenceStoreForm)
            }
        };

        column![
            tool_bar,
            container(content).height(Length::Fill),
            container(bottom_bar).height(Length::Shrink)
        ]
        .width(Length::Fill)
        .into()
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
            Message::AddNewEvidence => {
                self.processing_state = CasePageState::AddNewEvidence;
                None
            }
            Message::RecordToggled(record_id, is_checked) => {
                match is_checked {
                    true => {
                        self.selected_ids.insert(record_id);
                    }
                    false => {
                        self.selected_ids.remove(&record_id);
                    }
                }
                None
            }
            Message::SelectAllRecordsToggled(is_checked) => {
                match is_checked {
                    true => {
                        self.select_all_records = true;
                        self.selected_ids =
                            self.current_results.iter().map(|r| r.get_guid()).collect();
                    }
                    false => {
                        self.select_all_records = false;
                        self.selected_ids.clear();
                    }
                }
                None
            }
            Message::RecordSelected(id) => {
                self.selected_id = Some(id);
                None
            }
        }
    }

    fn results_widget(&self) -> Element<'_, Message> {
        row![
            container(self.info_panel_widget()).width(Length::FillPortion(1)),
            container(self.current_results()).width(Length::FillPortion(3)),
            container(self.selected_record_widget()).width(Length::FillPortion(1))
        ]
        .into()
    }

    fn current_results(&self) -> Element<'_, Message> {
        if self.current_results.len() == 0 {
            return text("No Results").into();
        }

        let mut columns = self
            .current_results
            .first()
            .unwrap()
            .fields
            .keys()
            .map(|field_id| {
                let field_id = field_id.clone();
                table::column(
                    text(field_id.to_string()), // header element
                    move |row: &QueryResult| -> Element<'_, Message> {
                        // cell view fn
                        let id = row.get_guid();
                        let is_selected = self.selected_id == Some(id);
                        mouse_area(container(text(row.get(&field_id).to_string())).style(
                            if is_selected {
                                container::primary
                            } else {
                                container::transparent
                            },
                        ))
                        .on_press(Message::RecordSelected(id))
                        .into()
                    },
                )
            })
            .collect::<Vec<_>>();

        let selected_columns = table::column(
            checkbox(self.select_all_records).on_toggle(Message::SelectAllRecordsToggled),
            move |row: &QueryResult| -> Element<'_, Message> {
                let id = row.get_guid();
                let is_checked = self.selected_ids.contains(&id);
                checkbox(is_checked)
                    .label("")
                    .on_toggle(move |checked| Message::RecordToggled(id, checked))
                    .into()
            },
        );

        columns.insert(0, selected_columns);

        scrollable(table(columns, &self.current_results)).into()
    }

    fn selected_record_widget(&self) -> Element<'_, Message> {
        // will want to eventually display a table of all possible metadata,
        // maybe a metadata profile selector.
        let selected_record = self
            .current_results
            .iter()
            .find(|r| Some(r.get_guid()) == self.selected_id);

        if let Some(record) = selected_record {
            column![
                text("Selected Record"),
                text(format!("GUID: {}", record.get_guid().to_string()))
            ]
            .into()
        } else {
            column![text("Select a record")].into()
        }
    }

    fn info_panel_widget(&self) -> Element<'_, Message> {
        text("Info Panel").into()
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

    ///Context Menus are on the dev roadmap for 0.15, for now will generate a basic
    /// button menu for development needs
    fn tool_bar_widget(&self) -> Element<'_, Message> {
        row![
            button("Add Case Evidence").on_press(Message::AddNewEvidence),
            button("Close Case").on_press(Message::CloseCase),
        ]
        .into()
    }

    fn bootom_bar_widget(&self) -> Element<'_, Message> {
        center(row![self.selected_records_widget()]).into()
    }

    /// Displays the number of selected records, center of bottom row
    fn selected_records_widget(&self) -> Element<'_, Message> {
        if self.selected_ids.len() == 0 {
            text("No Selected Records.").into()
        } else {
            text(format!(
                "Selected {} Records.",
                &self.selected_ids.iter().count()
            ))
            .into()
        }
    }
}
