use iced::Element;
use iced::widget::{button, center, column, container, row, text, text_input};

#[derive(Debug, Clone)]
pub struct EvidenceStoreForm {
    store_name: String,
    store_path: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Cancel,
    Create,
    StoreNameChanged(String),
    StorePathChanged(String),
}

pub enum EvidenceStoreMessage {
    None,
    Cancel,
    Create(enlighten::EvidenceStore),
}

impl EvidenceStoreForm {
    pub fn new() -> Self {
        Self {
            store_name: String::new(),
            store_path: String::new(),
        }
    }

    fn to_store(&self) -> enlighten::EvidenceStore {
        enlighten::EvidenceStore::new(self.store_name.clone(), self.store_path.clone()).unwrap()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let content = self.new_evidence_widget();

        center(content).into()
    }

    pub fn update(&mut self, message: Message) -> EvidenceStoreMessage {
        match message {
            Message::Cancel => EvidenceStoreMessage::Cancel,
            Message::Create => EvidenceStoreMessage::Create(self.to_store()),
            Message::StoreNameChanged(name) => {
                self.store_name = name;
                EvidenceStoreMessage::None
            }
            Message::StorePathChanged(path) => {
                self.store_path = path;
                EvidenceStoreMessage::None
            }
        }
    }

    fn new_evidence_widget(&self) -> Element<'_, Message> {
        column![
            container(row![
                text("Store Name"),
                text_input("", &self.store_name).on_input(Message::StoreNameChanged)
            ]),
            container(row![
                text("Store Path"),
                text_input("", &self.store_path).on_input(Message::StorePathChanged)
            ]),
            row![
                button("Cancel").on_press(Message::Cancel),
                button("Create").on_press(Message::Create),
            ]
        ]
        .max_width(400)
        .into()
    }
}
