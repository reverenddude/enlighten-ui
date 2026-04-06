use iced::widget::{button, column, row, text};
use iced::{Alignment, Center, Element};

#[derive(Default, Debug, Clone)]
pub struct HomePage {
    page_state: HomePageState,
    new_case_settings: NewCaseSettings,
    open_case_settings: OpenCaseSettings,
}

#[derive(Default, Debug, Clone)]
pub struct NewCaseSettings {
    pub case_name: String,
    pub case_path: String,
}

#[derive(Default, Debug, Clone)]
pub struct OpenCaseSettings {
    pub case_path: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    CreateNewCase,
    OpenCase,
    Submit,
    Cancel,

    NewCaseNameChanged(String),
    NewCasePathChanged(String),
    OpenCasePathChanged(String),
}

#[derive(Debug, Default, Clone)]
enum HomePageState {
    #[default]
    Home,
    CreateCase,
    OpenCase,
}

pub enum CaseSettings {
    NewCase(NewCaseSettings),
    OpenCase(String),
}

impl HomePage {
    pub fn new() -> Self {
        Self {
            page_state: HomePageState::Home,
            new_case_settings: NewCaseSettings::default(),
            open_case_settings: OpenCaseSettings::default(),
        }
    }

    pub fn update(&mut self, message: Message) -> Option<CaseSettings> {
        match message {
            Message::CreateNewCase => {
                self.page_state = HomePageState::CreateCase;
                None
            }
            Message::OpenCase => {
                self.page_state = HomePageState::OpenCase;
                None
            }
            Message::Submit => match self.page_state {
                HomePageState::CreateCase => {
                    Some(CaseSettings::NewCase(self.new_case_settings.clone()))
                }
                HomePageState::OpenCase => Some(CaseSettings::OpenCase(
                    self.open_case_settings.case_path.clone(),
                )),
                HomePageState::Home => None,
            },
            Message::NewCaseNameChanged(name) => {
                self.new_case_settings.case_name = name;
                None
            }
            Message::NewCasePathChanged(path) => {
                self.new_case_settings.case_path = path;
                None
            }
            Message::OpenCasePathChanged(path) => {
                self.open_case_settings.case_path = path;
                None
            }
            Message::Cancel => {
                self.page_state = HomePageState::Home;
                None
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        match self.page_state {
            HomePageState::Home => self.home_widget(),
            HomePageState::CreateCase => self.new_case_widget(),
            HomePageState::OpenCase => self.open_case_widget(),
        }
    }

    fn home_widget(&self) -> Element<'_, Message> {
        row![
            column![
                text("Hello from Home"),
                button("Create New Case").on_press(Message::NewCase(NewCaseSettings {
                    case_name: "NewCase".to_string(),
                    case_path: "/tmp/enlighten/cases".to_string()
                })),
                button("Open Existing Case").on_press(Message::OpenCase(
                    "/tmp/enlighten/cases/NewCase".to_string()
                ))
            ]
            .max_width(500)
            .align_x(Center)
        ]
        .align_y(Center)
        .into()
    }

    fn new_case_widget(&self) -> Element<'_, Message> {
        
    }

    fn open_case_widget(&self) -> Element<'_, Message> {
        todo!()
    }
}
