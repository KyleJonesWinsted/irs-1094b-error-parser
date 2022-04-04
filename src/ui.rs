use std::env::current_exe;
use std::process;

use iced::Command;
use iced::{button, executor, Application, Button, Container, Row, Text};
use irs_1094b_error_parser::InputPaths;

pub static OPEN_FILE_ARG: &str = "open_file_dialog";

#[derive(Debug)]
pub struct App {
    _paths: InputPaths,
    error_file_select_button_state: button::State,
}

impl Application for App {
    type Executor = executor::Default;

    type Message = Action;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            App {
                _paths: InputPaths::default(),
                error_file_select_button_state: button::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "IRS 1094B Error Parser".to_string()
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut iced::Clipboard,
    ) -> iced::Command<Self::Message> {
        match message {
            Action::ChooseFile => return self.choose_file(),
            Action::SetFile(path) => println!("Set path! {:?}", path),
        };
        Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        Container::new(
            Row::new().push(Text::new("Select Error File")).push(
                Button::new(
                    &mut self.error_file_select_button_state,
                    Text::new("Choose File..."),
                )
                .on_press(Action::ChooseFile),
            ),
        )
        .into()
    }
}

impl App {
    fn choose_file(&mut self) -> Command<Action> {
        async {
            let output = process::Command::new(current_exe().unwrap())
                .arg(OPEN_FILE_ARG)
                .output()
                .unwrap();
            let path = String::from_utf8(output.stdout).unwrap();
            Action::SetFile(path).into()
        }
        .into()
    }
}

#[derive(Debug, Clone)]
pub enum Action {
    ChooseFile,
    SetFile(String),
}
