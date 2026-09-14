use crate::gui::{fonts, image::load_image};

use crate::gui::theme::CrimsonPuppet;
use crate::models::config::app_config;
use iced::{Element, Subscription, Task, futures::SinkExt, stream};

use crate::{
    app::{
        controller::{self, AppEvent, GuiEvent},
        elog,
    },
    gui::{
        idle::Idle,
        player::{Message as PlayerMessage, PlayerApp},
        welcome::{Action, Message as WelcomeMessage, WelcomeApp},
    },
};

pub struct App {
    screen: Screen,
    theme: CrimsonPuppet,
}

enum Screen {
    Welcome(WelcomeApp),
    Player(PlayerApp),
    Idle(Idle),
    Void,
    Image(iced::widget::image::Handle),
}

#[derive(Debug)]
pub enum Message {
    FontLoaded(Result<(), iced::font::Error>),
    Welcome(WelcomeMessage),
    Player(PlayerMessage),
    Gui(GuiEvent),
    ImageLoaded(Result<Vec<u8>, String>),
}

fn gui_listener() -> impl iced::futures::Stream<Item = Message> {
    stream::channel(
        100,
        |mut output: iced::futures::channel::mpsc::Sender<Message>| async move {
            loop {
                let event = {
                    let mut rx = controller::gui_receiver().lock().await;
                    rx.recv().await
                };

                match event {
                    Some(event) => {
                        match output.send(Message::Gui(event)).await {
                            Err(err) => {
                                elog!(
                                    &format!("Failed to load video: {:?}", err).to_string(),
                                    "gui"
                                );
                            }
                            Ok(_) => {}
                        };
                    }
                    None => break,
                }
            }
        },
    )
}

impl App {
    pub fn new() -> (Self, Task<Message>) {
        let (config, task) = WelcomeApp::new();

        let is_dark =
            dark_light::detect().unwrap_or(dark_light::Mode::Dark) == dark_light::Mode::Dark;

        let initial_theme = if is_dark {
            CrimsonPuppet::CrimsonDark
        } else {
            CrimsonPuppet::CrimsonLight
        };

        let font_task = fonts::load().map(Message::FontLoaded);

        let config_task = task.map(Message::Welcome);

        (
            Self {
                screen: Screen::Welcome(config),
                theme: initial_theme,
            },
            Task::batch(vec![config_task, font_task]),
        )
    }

    pub fn title(&self) -> String {
        return "Puppet".to_string();
    }

    pub fn theme(&self) -> iced::Theme {
        self.theme.clone().to_iced_theme()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::run(gui_listener)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match (&mut self.screen, message) {
            (Screen::Welcome(config), Message::Welcome(msg)) => {
                let (task, action) = config.update(msg);

                match action {
                    Action::None => {}

                    Action::ConfigSaved(show_idle) => {
                        controller::send(AppEvent::ConfigSaved);
                        if show_idle {
                            self.screen = Screen::Idle(Idle::new());
                        } else {
                            self.screen = Screen::Void;
                        }
                    }
                }

                task.map(Message::Welcome)
            }

            (Screen::Player(player), Message::Player(msg)) => {
                let is_end_of_stream = matches!(msg, crate::gui::player::Message::EndOfStream);

                player.update(msg);

                if is_end_of_stream {
                    if app_config()
                        .expect("Config not initialized on welcome - this should not happen")
                        .show_idle
                    {
                        self.screen = Screen::Idle(Idle::new());
                    } else {
                        self.screen = Screen::Void;
                    }
                }

                Task::none()
            }

            (_, Message::Gui(GuiEvent::LoadVideo(path))) => {
                let mut player = PlayerApp::new();
                player.open(path);

                self.screen = Screen::Player(player);

                Task::none()
            }

            (_, Message::Gui(GuiEvent::LoadImage(url))) => load_image(url),

            (_, Message::Gui(GuiEvent::UnLoad)) => {
                if app_config()
                    .expect("Config not initialized on welcome - this should not happen")
                    .show_idle
                {
                    self.screen = Screen::Idle(Idle::new());
                } else {
                    self.screen = Screen::Void;
                }
                Task::none()
            }

            (_, Message::ImageLoaded(result)) => {
                match result {
                    Ok(bytes) => {
                        let image = iced::widget::image::Handle::from_bytes(bytes);

                        self.screen = Screen::Image(image);
                    }

                    Err(err) => {
                        elog!(&format!("Failed to load image: {}", err), "gui");
                    }
                }

                Task::none()
            }

            _ => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        match &self.screen {
            Screen::Welcome(config) => config.view().map(Message::Welcome),
            Screen::Player(player) => player.view().map(Message::Player),
            Screen::Idle(idle) => idle.view().map(|_| unreachable!()),
            Screen::Image(image) => iced::widget::image(image.clone()).into(),
            Screen::Void => iced::widget::space().into(),
        }
    }
}
