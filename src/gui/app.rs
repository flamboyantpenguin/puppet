use iced::{Element, Subscription, Task, futures::SinkExt, stream};

use crate::gui::fonts;
use crate::gui::theme::CrimsonPuppet;

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
}

#[derive(Debug)]
pub enum Message {
    FontLoaded(Result<(), iced::font::Error>),
    Welcome(WelcomeMessage),
    Player(PlayerMessage),
    Gui(GuiEvent),
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

                    Action::ConfigSaved => {
                        controller::send(AppEvent::ConfigSaved);
                        self.screen = Screen::Idle(Idle::new());
                    }
                }

                task.map(Message::Welcome)
            }

            (Screen::Player(player), Message::Player(msg)) => {
                let is_end_of_stream = matches!(msg, crate::gui::player::Message::EndOfStream);

                player.update(msg);

                if is_end_of_stream {
                    self.screen = Screen::Idle(Idle::new());
                }

                Task::none()
            }

            (_, Message::Gui(GuiEvent::LoadVideo(path))) => {
                let mut player = PlayerApp::new();
                player.open(path);

                self.screen = Screen::Player(player);

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
        }
    }
}
