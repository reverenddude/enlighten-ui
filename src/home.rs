use iced::widget::{button, column, row, text};
use iced::{Alignment, Center, Element};

#[derive(Default, Debug, Clone, Copy)]
pub struct Home {
    show_new_case_modal: bool,
}

#[derive(Debug, Clone)]
pub struct NewCaseSettings {
    pub case_name: String,
    pub case_path: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    NewCase(NewCaseSettings),
    OpenCase(String),
}

impl Home {
    pub fn view(&self) -> Element<'_, Message> {
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
}
