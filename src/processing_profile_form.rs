use iced::Alignment::Center;
use iced::Element;
use iced::widget::{
    button, center, checkbox, column, container, pick_list, row, text, text_editor, text_input,
};

#[derive(Debug, Clone)]
pub struct ProcessingProfileForm {
    name: String,
    thread_count: usize,
    generate_sha256: bool,
    password_input: text_editor::Content,
    password_bank: Vec<String>,
    mime_settings: MimeSettingsForm,
}

#[derive(Debug, Clone)]
pub struct MimeSettingsForm {}

#[derive(Debug, Clone)]
pub enum Message {
    Cancel,
    Create,
    ProfileNameChanged(String),
    ThreadCountChanged(usize),
    CalculateSha256Selected(bool),
    PasswordListChanged(text_editor::Action),
}

#[derive(Debug, Clone)]
pub enum ProfileMessage {
    None,
    Cancel,
    ProcessingProfileSaved(enlighten::ProcessingProfile),
}

impl ProcessingProfileForm {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            thread_count: 0,
            generate_sha256: false,
            password_input: text_editor::Content::new(),
            password_bank: Vec::new(),
            mime_settings: MimeSettingsForm {},
        }
    }

    pub fn from_profile(profile: &enlighten::ProcessingProfile) -> Self {
        let pws: Vec<String> = profile
            .password_bank
            .passwords
            .iter()
            .map(|p| p.0.clone())
            .collect();
        let pw_list: String = pws.join("\n");

        Self {
            name: profile.name.clone(),
            thread_count: profile.engine_settings.processing_threads,
            generate_sha256: profile.engine_settings.generate_sha256_hash,
            password_input: text_editor::Content::with_text(pw_list.as_str()),
            password_bank: pws,
            mime_settings: MimeSettingsForm {},
        }
    }

    pub fn to_profile(&self) -> enlighten::ProcessingProfile {
        let pws: Vec<enlighten::Password> = self
            .password_bank
            .iter()
            .map(|p| enlighten::Password::new(p.clone()))
            .collect();

        let mut pw_bank = enlighten::PasswordBank::new();
        pw_bank.add_many(pws);

        enlighten::ProcessingProfileBuilder::new(&self.name)
            .with_thread_count(self.thread_count)
            .with_password_bank(pw_bank)
            .build()
    }

    pub fn view(&self) -> Element<'_, Message> {
        self.new_profile_widget()
    }

    pub fn update(&mut self, message: Message) -> ProfileMessage {
        match message {
            Message::Cancel => ProfileMessage::Cancel,
            Message::Create => {
                let profile = self.to_profile();
                ProfileMessage::ProcessingProfileSaved(profile)
            }
            Message::ProfileNameChanged(name) => {
                self.name = name;
                ProfileMessage::None
            }
            Message::ThreadCountChanged(count) => {
                self.thread_count = count;
                ProfileMessage::None
            }
            Message::CalculateSha256Selected(checked) => {
                self.generate_sha256 = checked;
                ProfileMessage::None
            }
            Message::PasswordListChanged(action) => {
                self.password_input.perform(action);
                ProfileMessage::None
            }
        }
    }

    fn new_profile_widget(&self) -> Element<'_, Message> {
        let available_thread_counts: Vec<usize> = (2..11).collect();

        let content = column![
            container(row![
                text("Profile Name"),
                text_input(&self.name.clone(), &self.name)
                    .width(300)
                    .on_input(Message::ProfileNameChanged),
            ]),
            container(row![
                text("Thread Count"),
                pick_list(
                    available_thread_counts,
                    Some(self.thread_count),
                    Message::ThreadCountChanged
                )
            ]),
            container(row![
                text("Calculate SHA-256"),
                checkbox(self.generate_sha256).on_toggle(Message::CalculateSha256Selected)
            ]),
            container(row![
                text("Passwords"),
                text_editor(&self.password_input).on_action(Message::PasswordListChanged)
            ]),
            row![
                button("Create").on_press(Message::Create),
                button("Cancel").on_press(Message::Cancel),
            ]
            .spacing(10)
        ]
        .spacing(10)
        .align_x(Center)
        .max_width(400);
        center(content).into()
    }
}
