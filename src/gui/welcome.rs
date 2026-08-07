use crate::gui::assets::LOGO_BYTES;
use crate::gui::fonts;
use crate::gui::models::Config;
use crate::gui::theme::{PaletteExt, footer_button_style};
use crate::models::config::{AppConfig, CONFIG};
use iced::alignment::{Horizontal, Vertical};
use iced::border::Radius;
use iced::gradient::Linear;
use iced::widget::space::horizontal;
use iced::widget::{
    Space, button, checkbox, column, container, row, scrollable, svg, text, text_editor, text_input,
};
use iced::{Alignment, Background, Border, Color, Element, Font, Gradient, Length, Task, Theme};

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
    ToggleLeftPane,
    JsonTextChanged(text_editor::Action),
}

pub struct WelcomeApp {
    config: Config,
    show_left_pane: bool,
    pub raw_json: text_editor::Content,
    pub json_error: Option<String>,
}

pub enum Action {
    None,
    ConfigSaved,
}

impl WelcomeApp {
    pub fn new() -> (Self, Task<Message>) {
        let config = Config::default();
        let initial_json = serde_json::to_string_pretty(&config).unwrap_or_default();
        let raw_json = text_editor::Content::with_text(&initial_json);

        (
            Self {
                config,
                show_left_pane: true,
                raw_json,
                json_error: None,
            },
            Task::none(),
        )
    }

    fn sync_json_from_config(&mut self) {
        if let Ok(pretty) = serde_json::to_string_pretty(&self.config) {
            self.raw_json = text_editor::Content::with_text(&pretty);
        }
    }

    pub fn update(&mut self, message: Message) -> (Task<Message>, Action) {
        match message {
            Message::HeaderChanged(val) => {
                self.config.header = val;
                self.sync_json_from_config();
            }
            Message::IdChanged(val) => {
                self.config.id = val;
                self.sync_json_from_config();
            }
            Message::DelayChanged(val) => {
                self.config.delay_ms = val;
                self.sync_json_from_config();
            }
            Message::TokenChanged(val) => {
                self.config.token = val;
                self.sync_json_from_config();
            }
            Message::PortChanged(val) => {
                self.config.port = val;
                self.sync_json_from_config();
            }
            Message::OneAtATimeToggled(val) => {
                self.config.one_at_a_time = val;
                self.sync_json_from_config();
            }
            Message::TimeHashToggled(val) => {
                self.config.time_hash_token = val;
                self.sync_json_from_config();
            }
            Message::SaveConfig => {
                let parsed_config = AppConfig {
                    header: self.config.header.clone(),
                    id: self.config.id.clone(),
                    delay_ms: self.config.delay_ms.parse().unwrap_or(0),
                    token: self.config.token.clone(),
                    port: self.config.port.parse().unwrap_or(8888),
                    one_at_a_time: self.config.one_at_a_time,
                    time_hash_token: self.config.time_hash_token,
                };
                let _ = CONFIG.set(parsed_config);
                return (Task::none(), Action::ConfigSaved);
            }

            Message::JsonTextChanged(action) => {
                let is_edit = action.is_edit();

                self.raw_json.perform(action);
                if is_edit {
                    let data = self.raw_json.text();

                    match serde_json::from_str::<Config>(&data) {
                        Ok(parsed_config) => {
                            self.config = parsed_config;
                            self.json_error = None;
                        }
                        Err(err) => {
                            self.json_error = Some(format!("Syntax Error: {err}"));
                        }
                    }
                }
            }

            Message::ToggleLeftPane => self.show_left_pane = !self.show_left_pane,
        }
        (Task::none(), Action::None)
    }

    pub fn view(&self) -> Element<'_, Message> {
        let title = text("Puppet").size(30).style(text::primary);

        let pane_toggle = button(text("")).on_press(Message::ToggleLeftPane);

        let header_input = column![
            text("Header:").size(14).style(text::secondary),
            text_input("Enter Header", &self.config.header)
                .on_input(Message::HeaderChanged)
                .size(16)
                .padding([12, 14])
        ]
        .spacing(6);

        let id_input = column![
            text("ID:").size(14).style(text::secondary),
            text_input("Enter ID", &self.config.id)
                .on_input(Message::IdChanged)
                .size(16)
                .padding([12, 14])
        ]
        .spacing(6);

        let delay_input = column![
            text("Delay (ms):").size(14).style(text::secondary),
            text_input("100", &self.config.delay_ms.to_string())
                .on_input(Message::DelayChanged)
                .size(16)
                .padding([12, 14])
        ]
        .spacing(6);

        let token_input = column![
            text("Token:").size(14).style(text::secondary),
            text_input("Enter auth token", &self.config.token)
                .on_input(Message::TokenChanged)
                .secure(true)
                .size(16)
                .padding([12, 14])
        ]
        .spacing(6);

        let port_input = column![
            text("Port:").size(14).style(text::secondary),
            text_input("8080", &self.config.port.to_string())
                .on_input(Message::PortChanged)
                .size(16)
                .padding([10, 12])
                .width(Length::Fixed(120.0))
        ]
        .spacing(6);

        let sync_toggle = checkbox(self.config.one_at_a_time)
            .label("Process one message at a time")
            .font(Font::MONOSPACE)
            .on_toggle(Message::OneAtATimeToggled)
            .size(20)
            .spacing(12);

        let time_hash_toggle = checkbox(self.config.time_hash_token)
            .label("Turn on token hashing to prevent intruders")
            .font(Font::MONOSPACE)
            .on_toggle(Message::TimeHashToggled)
            .size(20)
            .spacing(12);

        let handle = svg::Handle::from_memory(LOGO_BYTES);

        let logo = svg(handle)
            .width(200)
            .height(200)
            .style(|theme: &Theme, _status| {
                let palette = theme.palette();

                svg::Style {
                    color: Some(palette.primary),
                }
            });

        let left_pane = if self.show_left_pane {
            container(
                column![
                    scrollable(
                        column![
                            row![title, horizontal(), pane_toggle].align_y(Alignment::Center),
                            header_input,
                            id_input,
                            delay_input,
                            token_input,
                            port_input,
                            sync_toggle,
                            time_hash_toggle,
                        ]
                        .spacing(20)
                    )
                    .height(Length::Fill),
                    row![logo, horizontal()]
                ]
                .spacing(20),
            )
            .width(Length::FillPortion(1))
            .height(Length::Fill)
            .padding(24)
            .style(|theme: &Theme| {
                let palette = theme.palette();

                container::Style {
                    border: Border {
                        color: palette.secondary(),
                        radius: Radius::new(10),
                        width: 2.0,
                    },
                    ..Default::default()
                }
            })
        } else {
            container(Space::new())
                .height(Length::Fill)
                .center_y(Length::Fill)
                .padding(12)
        };

        let top_right = container(
            row![
                text("Le monde entier est un théâtre")
                    .size(24)
                    .style(text::secondary),
                horizontal(),
                button(text(" Edit").font(fonts::NERD_FONT).size(24))
                    .style(if self.show_left_pane {
                        button::secondary
                    } else {
                        button::subtle
                    })
                    .on_press(Message::ToggleLeftPane),
                button(text(" Run").font(fonts::NERD_FONT).size(24))
                    .style(button::primary)
                    .on_press(Message::SaveConfig)
            ]
            .spacing(10),
        )
        .width(Length::Fill);

        let middle_right = container(
            text_editor(&self.raw_json)
                .font(Font::MONOSPACE)
                .style(|theme: &Theme, _status| {
                    let palette = theme.palette();

                    text_editor::Style {
                        background: Background::Color(palette.background),
                        border: Border {
                            color: palette.secondary(),
                            radius: Radius::new(10),
                            width: 2.0,
                        },
                        placeholder: palette.secondary(),
                        value: palette.text,
                        selection: palette.primary,
                    }
                })
                .on_action(Message::JsonTextChanged)
                .font(fonts::NERD_FONT)
                .height(Length::Fill),
        )
        .height(Length::Fill);

        let github_button = button(text("").font(fonts::NERD_FONT).size(24))
            .style(footer_button_style)
            .on_press(Message::SaveConfig);

        let packages_button = button(text("").font(fonts::NERD_FONT).size(24))
            .style(footer_button_style)
            .on_press(Message::SaveConfig);

        let puppeteer_button = button(text("󰮃").font(fonts::NERD_FONT).size(24))
            .style(footer_button_style)
            .on_press(Message::SaveConfig);

        let bottom_right = container(
            row![
                github_button,
                packages_button,
                puppeteer_button,
                horizontal(),
                text("Puppet Pre-󰀫").font(fonts::NERD_FONT)
            ]
            .align_y(Vertical::Center),
        )
        .width(Length::Fill)
        .padding(iced::Padding {
            top: 0.0,
            right: 10.0,
            bottom: 0.0,
            left: 0.0,
        })
        .align_x(Horizontal::Right)
        .style(|theme: &Theme| {
            let palette = theme.palette();

            let linear_gradient = Linear::new(0.785)
                .add_stop(0.0, palette.primary)
                .add_stop(1.0, palette.tertiary());

            container::Style {
                text_color: Some(palette.text),
                background: Some(Background::Gradient(Gradient::Linear(linear_gradient))),
                border: Border {
                    color: Color::TRANSPARENT,
                    radius: Radius::new(10),
                    width: 2.0,
                },
                ..Default::default()
            }
        });

        let mut right_column = column![top_right, middle_right, bottom_right]
            .width(Length::FillPortion(3))
            .height(Length::Fill)
            .spacing(12);

        if let Some(err) = &self.json_error {
            right_column = right_column.push(text(err).size(13).font(Font::MONOSPACE));
        }

        container(row![left_pane, right_column].spacing(20))
            .align_top(Length::Fill)
            .align_left(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .padding(20)
            .into()
    }
}
