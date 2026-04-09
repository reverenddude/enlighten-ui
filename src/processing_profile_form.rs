use iced::Element;
use iced::widget::{button, column, container, row, text, text_input};

#[derive(Debug, Clone)]
pub struct ProcessingProfileForm {
    name: String,
    thread_count: usize,
    generate_sha256: bool,
    password_bank: PasswordBankForm,
    mime_settings: MimeSettingsForm,
}

#[derive(Debug, Clone)]
pub struct PasswordBankForm {}

#[derive(Debug, Clone)]
pub struct MimeSettingsForm {}

#[derive(Debug, Clone)]
pub enum Message {
    Cancel,
    Save,
    Submit,
    ProfileNameChanged(String),
}

#[derive(Debug, Clone)]
pub enum ProfileMessage {
    None,
    ProcessingProfileSaved(enlighten::ProcessingProfile),
}

impl ProcessingProfileForm {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            thread_count: 0,
            generate_sha256: false,
            password_bank: PasswordBankForm {},
            mime_settings: MimeSettingsForm {},
        }
    }

    pub fn from_profile(profile: &enlighten::ProcessingProfile) -> Self {
        Self {
            name: profile.name.clone(),
            thread_count: profile.engine_settings.processing_threads,
            generate_sha256: profile.engine_settings.generate_sha256_hash,
            password_bank: PasswordBankForm {},
            mime_settings: MimeSettingsForm {},
        }
    }

    pub fn to_profile(&self) -> enlighten::ProcessingProfile {
        enlighten::ProcessingProfileBuilder::new(&self.name)
            .with_thread_count(self.thread_count)
            .build()
    }

    pub fn view(&self) -> Element<'_, Message> {
        self.new_profile_widget()
    }

    pub fn update(&mut self, message: Message) -> ProfileMessage {
        match message {
            Message::Cancel => ProfileMessage::None,
            Message::Save => {
                let profile = self.to_profile();
                ProfileMessage::ProcessingProfileSaved(profile)
            }
            _ => ProfileMessage::None,
        }
    }

    fn new_profile_widget(&self) -> Element<'_, Message> {
        column![
            container(row![
                text("Profile Name"),
                text_input(&self.name.clone(), &self.name)
                    .width(300)
                    .on_input(Message::ProfileNameChanged),
            ]),
            row![
                button("Cancel").on_press(Message::Cancel),
                button("Submit").on_press(Message::Submit)
            ]
            .spacing(10)
        ]
        .into()
    }
}
