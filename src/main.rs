use iced::widget::{column, container, pick_list, row, text};
use iced::{Element, Length, Task, window};

mod case;
mod home;

use case::CasePage;
use home::HomePage;

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .window(window::Settings {
            position: window::Position::Centered,
            ..Default::default()
        })
        .theme(|app: &App| app.theme.clone())
        .title(App::title)
        .run()
}

///Application State. Knows about how to render the basic views, or how the sub pages handle views and updates.
///App.update handles the incoming messages and updates the state accordingly.
///App.view renders the current page.
struct App {
    current_page: Page,
    theme: iced::Theme,
}

impl Default for App {
    fn default() -> Self {
        Self {
            current_page: Page::Home(HomePage::default()),
            theme: iced::Theme::Dark,
        }
    }
}

impl App {
    fn title(&self) -> String {
        match self.current_page {
            Page::Home(..) => "Enlighten".to_string(),
            Page::Case(ref case_page) => case_page.case_name(),
        }
    }

    // Messages are just commands the user initiates via interactions
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Home(home_msg) => {
                if let Page::Home(home_page) = &mut self.current_page {
                    let hp_result = home_page.update(home_msg);
                    match hp_result {
                        home::HomePageUpdateResult::Task(task) => {
                            return task.map(Message::Home);
                        }
                        home::HomePageUpdateResult::Navigate(case_settings) => {
                            match case_settings {
                                home::CaseSettings::NewCase(new_case_settings) => {
                                    let case = enlighten::Case::new(
                                        new_case_settings.case_name,
                                        new_case_settings.case_path,
                                    )
                                    .unwrap();
                                    self.current_page = Page::Case(CasePage::new(case));
                                }
                                home::CaseSettings::OpenCase(case_path) => {
                                    println!("Opening case: {}", case_path);
                                    let case = enlighten::Case::open(case_path).unwrap();
                                    self.current_page = Page::Case(CasePage::new(case));
                                }
                            };
                        }
                        home::HomePageUpdateResult::None => {}
                    }
                }
                Task::none()
            }
            Message::Case(case_msg) => {
                if let Page::Case(case_page) = &mut self.current_page {
                    if let Some(nav) = case_page.update(case_msg) {
                        match nav {
                            case::NavigationMessage::CloseCase => {
                                self.current_page = Page::Home(HomePage::new());
                            }
                        }
                    }
                }
                Task::none()
            }
            Message::SetTheme(theme) => {
                self.theme = theme;
                Task::none()
            }
        }
    }

    // Anything that returns an Element<> is a widget in Iced.
    // So the Application view is really just a widget that displays other widgets.
    fn view(&self) -> Element<'_, Message> {
        let content: Element<Message> = match self.current_page {
            Page::Home(ref home_page) => home_page.view().map(Message::Home),
            Page::Case(ref case_page) => case_page.view().map(Message::Case),
        };

        //center(content).into()
        column![container(content).height(Length::Fill), self.theme_picker()].into()
    }

    fn theme_picker(&self) -> Element<'_, Message> {
        let themes = iced::Theme::ALL;

        row![
            text("DEVLOPMENT ONLY"),
            pick_list(themes, Some(&self.theme), Message::SetTheme)
        ]
        .into()
    }
}

#[derive(Debug)]
enum Page {
    Home(HomePage),
    Case(CasePage),
}

impl Default for Page {
    fn default() -> Self {
        Self::Home(HomePage::new())
    }
}

#[derive(Debug, Clone)]
enum Message {
    Home(home::Message),
    Case(case::Message),
    SetTheme(iced::Theme),
}
