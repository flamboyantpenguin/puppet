use iced::{Element, Task};

use crate::{
    app::controller::{self, AppEvent},
    gui::config::{Action, ConfigApp, Message as ConfigMessage},
};

pub struct App {
    screen: Screen,
}

enum Screen {
    Config(ConfigApp),
}

#[derive(Debug, Clone)]
pub enum Message {
    Config(ConfigMessage),
}

impl App {
    pub fn new() -> (Self, Task<Message>) {
        let (config, task) = ConfigApp::new();

        (
            Self {
                screen: Screen::Config(config),
            },
            task.map(Message::Config),
        )
    }

    pub fn title(&self) -> String {
        return "Victim".to_string();
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match (&mut self.screen, message) {
            (Screen::Config(config), Message::Config(msg)) => {
                let (task, action) = config.update(msg);

                match action {
                    Action::None => {}

                    Action::ConfigSaved => {
                        controller::send(AppEvent::ConfigSaved);
                    }
                }

                task.map(Message::Config)
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        match &self.screen {
            Screen::Config(config) => config.view().map(Message::Config),
        }
    }
}
