use crate::models::config::{AppConfig, CONFIG};
use iced::widget::{button, checkbox, column, container, text, text_input};
use iced::{Element, Length, Task};

#[derive(Debug, Clone)]
pub enum Message {
    HeaderChanged(String),
    IdChanged(String),
    DelayChanged(String),
    TokenChanged(String),
    PortChanged(String),
    OneAtATimeToggled(bool),
    TimeHashToggled(bool),
    SaveConfig,
}

pub struct ConfigApp {
    header: String,
    id: String,
    delay_ms: String,
    token: String,
    port: String,
    one_at_a_time: bool,
    time_hash_token: bool,
    saved: bool,
}

pub enum Action {
    None,
    ConfigSaved,
}

impl ConfigApp {
    pub fn new() -> (Self, Task<Message>) {
        let (header, id, delay_ms, token, port, one_at_a_time, time_hash_token) =
            if let Some(config) = CONFIG.get() {
                (
                    config.header.clone(),
                    config.id.clone(),
                    config.delay_ms.to_string(),
                    config.token.clone(),
                    config.port.to_string(),
                    config.one_at_a_time,
                    config.time_hash_token,
                )
            } else {
                let temp = AppConfig::gen_sample();
                (
                    temp.header,
                    temp.id,
                    temp.delay_ms.to_string(),
                    temp.token,
                    temp.port.to_string(),
                    temp.one_at_a_time,
                    temp.time_hash_token,
                )
            };

        (
            Self {
                header,
                id,
                delay_ms,
                token,
                port,
                one_at_a_time,
                time_hash_token,
                saved: false,
            },
            Task::none(),
        )
    }

    pub fn update(&mut self, message: Message) -> (Task<Message>, Action) {
        match message {
            Message::HeaderChanged(val) => self.id = val,
            Message::IdChanged(val) => self.id = val,
            Message::DelayChanged(val) => self.delay_ms = val,
            Message::TokenChanged(val) => self.token = val,
            Message::PortChanged(val) => self.port = val,
            Message::OneAtATimeToggled(val) => self.one_at_a_time = val,
            Message::TimeHashToggled(val) => self.time_hash_token = val,
            Message::SaveConfig => {
                let parsed_config = AppConfig {
                    header: self.header.clone(),
                    id: self.id.clone(),
                    delay_ms: self.delay_ms.parse::<u64>().unwrap_or(0),
                    token: self.token.clone(),
                    port: self.port.parse::<u64>().unwrap_or(8080),
                    one_at_a_time: self.one_at_a_time,
                    time_hash_token: self.time_hash_token,
                };

                let _ = CONFIG.set(parsed_config);

                self.saved = true;

                return (Task::none(), Action::ConfigSaved);
            }
        }
        (Task::none(), Action::None)
    }

    pub fn view(&self) -> Element<'_, Message> {
        let title = text("Configuration Setup").size(24);

        let header_input = column![
            text("Header:"),
            text_input("Enter Header", &self.header).on_input(Message::HeaderChanged)
        ];

        let id_input = column![
            text("ID:"),
            text_input("Enter ID", &self.id).on_input(Message::IdChanged)
        ];

        let delay_input = column![
            text("Delay (ms):"),
            text_input("100", &self.delay_ms).on_input(Message::DelayChanged)
        ];

        let token_input = column![
            text("Token:"),
            text_input("Enter auth token", &self.token)
                .on_input(Message::TokenChanged)
                .secure(true)
        ];

        let port_input = column![
            text("Port:"),
            text_input("8080", &self.port).on_input(Message::PortChanged)
        ];

        let sync_toggle = checkbox(self.one_at_a_time)
            .label("Process one message at a time")
            .on_toggle(Message::OneAtATimeToggled);

        let time_hash_toggle = checkbox(self.time_hash_token)
            .label("Turn on token hashing to prevent intruders")
            .on_toggle(Message::TimeHashToggled);

        let save_btn = button("Loose Control!").on_press(Message::SaveConfig);

        let mut content = column![
            title,
            header_input,
            id_input,
            delay_input,
            token_input,
            port_input,
            sync_toggle,
            time_hash_toggle,
            save_btn,
        ]
        .spacing(12)
        .max_width(400);

        if self.saved {
            content = content.push(text("Config successfully committed to CONFIG!").size(14));
        }

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .padding(20)
            .into()
    }
}
