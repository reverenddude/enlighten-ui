use iced::alignment::Horizontal::Right;
use iced::alignment::Vertical::Bottom;
use iced::widget::{button, column, container, row, text, text_input};
use iced::{Center, Element, Length};

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
            // Case Actions
            column![
                container(text("Enlighten Processing").height(Length::FillPortion(3))),
                container(self.case_btns())
                    .align_y(Bottom)
                    .align_x(Center)
                    .height(Length::FillPortion(1))
            ]
            .height(Length::FillPortion(1))
            .align_x(Center),
            // Recent Case List
            container(self.recent_case_list())
                .style(container::rounded_box)
                .align_x(Right)
                .height(Length::Fill)
                .width(Length::Fill)
        ]
        .into()
    }

    fn recent_case_list(&self) -> Element<'_, Message> {
        column![text("Recent Cases")].spacing(10).into()
    }

    fn case_btns(&self) -> Element<'_, Message> {
        row![
            button("Create New Case")
                .height(80)
                .on_press(Message::CreateNewCase),
            button("Open Existing Case")
                .height(80)
                .on_press(Message::OpenCase)
        ]
        .spacing(10)
        .into()
    }

    fn new_case_widget(&self) -> Element<'_, Message> {
        row![
            text("Create New Case"),
            column![
                row![
                    text("Case Name"),
                    text_input("", &self.new_case_settings.case_name)
                        .on_input(Message::NewCaseNameChanged)
                ],
                row![
                    text("Case Path"),
                    text_input("", &self.new_case_settings.case_path)
                        .on_input(Message::NewCasePathChanged)
                ],
                button("Create Case").on_press(Message::Submit),
                self.cancel_btn()
            ]
        ]
        .into()
    }

    fn open_case_widget(&self) -> Element<'_, Message> {
        row![
            text("Open Case"),
            column![
                row![
                    text("Case Path"),
                    text_input("", &self.open_case_settings.case_path)
                        .on_input(Message::OpenCasePathChanged)
                ],
                button("Open Case").on_press(Message::Submit),
                self.cancel_btn()
            ]
        ]
        .into()
    }

    fn cancel_btn(&self) -> Element<'_, Message> {
        button("Cancel").on_press(Message::Cancel).into()
    }
}
