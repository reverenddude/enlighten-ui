use iced::Alignment::Center;
use iced::Element;
use iced::widget::{button, center, column, row, text};

mod home;
mod processing;
use home::Home;
use processing::Processing;

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view).run()
}

struct App {
    current_page: Page,
}

impl App {
    fn new() -> Self {
        Self {
            current_page: Page::Home,
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Processing(processing_msg) => match processing_msg {
                processing::Message::DisplayCase => self.current_page = Page::Case,
            },
            Message::NewCase => self.current_page = Page::Processing,
            Message::Home(home_msg) => match home_msg {
                home::Message::Loaded => self.current_page = Page::Home,
                home::Message::NewCase => self.current_page = Page::Processing,
                home::Message::OpenCase => self.current_page = Page::Case,
            },
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let content: Element<Message> = match self.current_page {
            Page::Home => Home.view().map(Message::Home),
            Page::Processing => Processing.view().map(Message::Processing),
            Page::Case => row![
                text("Hello from case"),
                button("Close Case").on_press(Message::Home(home::Message::Loaded))
            ]
            .align_y(Center)
            .into(),
        };

        center(content).into()
    }
}

enum Page {
    Home,
    Processing,
    Case,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Home(home::Message),
    Processing(processing::Message),
    NewCase,
}
