use iced::Element;
use iced::widget::center;

mod case;
mod home;
mod processing;

use case::Case;
use home::Home;

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view).run()
}

#[derive(Default)]
struct App {
    current_page: Page,
    home: Home,
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::Home(home_msg) => match home_msg {
                home::Message::NewCase(new_case_settings) => {
                    let case = enlighten::Case::new(
                        new_case_settings.case_name,
                        new_case_settings.case_path,
                    )
                    .unwrap();
                    self.current_page = Page::Case(case)
                }
                home::Message::OpenCase(case_path) => {
                    println!("Opening case: {}", case_path);
                    let case = enlighten::Case::open(case_path).unwrap();
                    self.current_page = Page::Case(case);
                }
            },
            Message::Case(case_msg) => match case_msg {
                case::Message::CloseCase => self.current_page = Page::Home,
            },
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let content: Element<Message> = match self.current_page {
            Page::Home => self.home.view().map(Message::Home),
            Page::Case(ref enlighten_case) => Case.view(enlighten_case).map(Message::Case),
        };

        center(content).into()
    }
}

#[derive(Default)]
enum Page {
    #[default]
    Home,
    Case(enlighten::Case),
}

#[derive(Debug, Clone)]
enum Message {
    Home(home::Message),
    Case(case::Message),
}
