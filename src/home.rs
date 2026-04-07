use std::path::PathBuf;

use iced::alignment::Horizontal::Right;
use iced::alignment::Vertical::Bottom;
use iced::widget::{button, column, container, row, space, text, text_input};
use iced::{Center, Element, Length, Task};

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

    // Open Case Browser
    BrowseClicked,
    FolderSelected(Option<PathBuf>),
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

pub enum HomePageUpdateResult {
    Task(Task<Message>),
    Navigate(CaseSettings),
    None,
}

impl HomePage {
    pub fn new() -> Self {
        Self {
            page_state: HomePageState::Home,
            new_case_settings: NewCaseSettings::default(),
            open_case_settings: OpenCaseSettings::default(),
        }
    }

    pub fn update(&mut self, message: Message) -> HomePageUpdateResult {
        match message {
            Message::CreateNewCase => {
                self.page_state = HomePageState::CreateCase;
                HomePageUpdateResult::None
            }
            Message::OpenCase => {
                self.page_state = HomePageState::OpenCase;
                HomePageUpdateResult::None
            }
            Message::Submit => match self.page_state {
                HomePageState::CreateCase => HomePageUpdateResult::Navigate(CaseSettings::NewCase(
                    self.new_case_settings.clone(),
                )),
                HomePageState::OpenCase => HomePageUpdateResult::Navigate(CaseSettings::OpenCase(
                    self.open_case_settings.case_path.clone(),
                )),
                HomePageState::Home => HomePageUpdateResult::None,
            },
            Message::NewCaseNameChanged(name) => {
                self.new_case_settings.case_name = name;
                HomePageUpdateResult::None
            }
            Message::NewCasePathChanged(path) => {
                self.new_case_settings.case_path = path;
                HomePageUpdateResult::None
            }
            Message::OpenCasePathChanged(path) => {
                self.open_case_settings.case_path = path;
                HomePageUpdateResult::None
            }
            Message::Cancel => {
                self.page_state = HomePageState::Home;
                HomePageUpdateResult::None
            }
            Message::BrowseClicked => {
                let task = Task::perform(
                    async {
                        rfd::AsyncFileDialog::new()
                            .set_title("Select Case Folder")
                            .pick_folder()
                            .await
                            .map(|handle| handle.path().to_path_buf())
                    },
                    Message::FolderSelected,
                );
                HomePageUpdateResult::Task(task)
            }
            Message::FolderSelected(path) => match self.page_state {
                HomePageState::OpenCase => {
                    if let Some(path) = path {
                        self.open_case_settings.case_path = path.to_string_lossy().to_string();
                        return HomePageUpdateResult::Navigate(CaseSettings::OpenCase(
                            self.open_case_settings.case_path.clone(),
                        ));
                    } else {
                        return HomePageUpdateResult::None;
                    }
                }
                HomePageState::CreateCase => {
                    if let Some(path) = path {
                        self.new_case_settings.case_path = path.to_string_lossy().to_string();
                    };
                    return HomePageUpdateResult::None;
                }
                HomePageState::Home => {
                    return HomePageUpdateResult::None;
                }
            },
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
                container(text("Enlighten Processing").height(Length::FillPortion(5))),
                container(self.case_btns())
                    .padding([0, 20])
                    .align_y(Bottom)
                    .align_x(Center)
                    .height(Length::FillPortion(2)),
                space().height(Length::FillPortion(1))
            ]
            .height(Length::FillPortion(1))
            .align_x(Center),
            // Recent Case Lis
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
                row![
                    button("Browse").on_press(Message::BrowseClicked),
                    button("Create Case").on_press(Message::Submit),
                    self.cancel_btn()
                ]
            ]
        ]
        .spacing(10)
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
                ]
                .spacing(10),
                row![
                    button("Submit").on_press(Message::Submit),
                    button("Browse").on_press(Message::BrowseClicked),
                    self.cancel_btn()
                ]
                .spacing(10)
            ]
        ]
        .spacing(10)
        .into()
    }

    fn cancel_btn(&self) -> Element<'_, Message> {
        button("Cancel").on_press(Message::Cancel).into()
    }
}
