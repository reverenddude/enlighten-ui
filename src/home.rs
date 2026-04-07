use std::path::PathBuf;

use iced::alignment::Horizontal::Right;
use iced::alignment::Vertical::Bottom;
use iced::widget::{button, center, column, container, row, space, text, text_input};
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
        let left_panel = column![
            container(text("Enlighten Processing").height(Length::FillPortion(5))),
            container(self.case_btns())
                .padding([0, 20])
                .align_y(Bottom)
                .align_x(Center)
                .height(Length::FillPortion(2)),
            space().height(Length::FillPortion(1))
        ]
        .height(Length::FillPortion(1))
        .align_x(Center);

        let right_panel = match self.page_state {
            HomePageState::Home => container(column![text("Recent Cases")].spacing(10))
                .style(container::rounded_box)
                .align_x(Center)
                .height(Length::Fill)
                .width(Length::Fill)
                .into(),
            HomePageState::CreateCase => self.new_case_widget(),
            HomePageState::OpenCase => self.open_case_widget(),
        };

        let content = row![left_panel, right_panel];

        center(content).into()
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
        let content = column![
            text("Create New Case"),
            // Case Name Entry
            container(
                row![
                    container(text("Case Name")).padding([0, 10]),
                    text_input("", &self.new_case_settings.case_name)
                        .width(300)
                        .on_input(Message::NewCaseNameChanged)
                ]
                .align_y(Center)
            )
            .style(container::rounded_box),
            // Case Path Entry
            container(
                row![
                    container(text("Case Path")).padding([0, 10]),
                    text_input("", &self.new_case_settings.case_path)
                        .width(300)
                        .on_input(Message::NewCasePathChanged)
                ]
                .align_y(Center)
            )
            .style(container::rounded_box),
            // Action Buttons
            row![
                button("Browse").on_press(Message::BrowseClicked),
                button("Create Case").on_press(Message::Submit),
                self.cancel_btn()
            ]
            .spacing(10)
        ]
        .align_x(Center)
        .max_width(500)
        .spacing(10);

        center(content).into()
    }

    // todo - rethink this widgeth entirly.. should probably just display
    // the folder browser and attempt to open the case instead of being on a seperate
    // view
    fn open_case_widget(&self) -> Element<'_, Message> {
        let content = column![
            text("Open Existing Case"),
            container(
                row![
                    container(text("Case Path")).padding([0, 10]),
                    text_input("", &self.open_case_settings.case_path)
                        .width(300)
                        .on_input(Message::OpenCasePathChanged)
                ]
                .align_y(Center)
            )
            .style(container::rounded_box),
            row![
                button("Submit").on_press(Message::Submit),
                button("Browse").on_press(Message::BrowseClicked),
                self.cancel_btn()
            ]
            .spacing(10)
        ]
        .align_x(Center)
        .max_width(500)
        .spacing(10);

        center(content).into()
    }

    fn cancel_btn(&self) -> Element<'_, Message> {
        button("Cancel").on_press(Message::Cancel).into()
    }
}
