use iced::Element;
use iced::widget::center;

mod case;
mod home;
mod processing;

use case::CasePage;
use home::HomePage;

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title(App::title)
        .run()
}

///Application State. Knows about how to render the basic views, or how the sub pages handle views and updates.
///App.update handles the incoming messages and updates the state accordingly.
///App.view renders the current page.
#[derive(Default)]
struct App {
    current_page: Page,
}

impl App {
    fn title(&self) -> String {
        match self.current_page {
            Page::Home(..) => "Enlighten".to_string(),
            Page::Case(ref case_page) => case_page.case_name(),
        }
    }

    // Messages are just commands the user initiates via interactions
    fn update(&mut self, message: Message) {
        match message {
            Message::Home(home_msg) => {
                if let Page::Home(home_page) = &mut self.current_page {
                    if let Some(case) = home_page.update(home_msg) {
                        match case {
                            home::CaseSettings::NewCase(new_case_settings) => {
                                let case = enlighten::Case::new(
                                    new_case_settings.case_name,
                                    new_case_settings.case_path,
                                )
                                .unwrap();
                                self.current_page = Page::Case(CasePage::new(case))
                            }
                            home::CaseSettings::OpenCase(case_path) => {
                                println!("Opening case: {}", case_path);
                                let case = enlighten::Case::open(case_path).unwrap();
                                self.current_page = Page::Case(CasePage::new(case));
                            }
                        }
                    }
                }
            }

            // The case sub page generates numerous internal messages and a singular navigation message.
            // The application only needs to care about the navigation message as it relates to the applciation
            // rendering pages.
            Message::Case(case_msg) => {
                if let Page::Case(case_page) = &mut self.current_page {
                    if let Some(nav) = case_page.update(case_msg) {
                        match nav {
                            case::NavigationMessage::CloseCase => {
                                self.current_page = Page::Home(HomePage::new())
                            }
                        }
                    }
                }
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
        content.into()
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
}
