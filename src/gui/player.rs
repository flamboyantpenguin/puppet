use iced::{
    Element, Event, Subscription, event,
    keyboard::{self, key},
    widget::{Column, Container, Text},
};
use iced_video_player::{Video, VideoPlayer};
use std::{path::Path, time::Duration};
use tokio::time::Instant;

use crate::{
    app::{blog, elog},
    models::config::app_static,
};

#[derive(Clone)]
pub enum Message {
    EndOfStream,
    NewFrame,
    Event(Event),
}

pub struct PlayerApp {
    video: Option<Video>,
    position: f64,
    dragging: bool,
    last_r_press: Option<Instant>,
}

impl PlayerApp {
    pub fn new() -> Self {
        Self {
            video: None,
            position: 0.0,
            dragging: false,
            last_r_press: None,
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::Event)
    }

    pub fn open(&mut self, path: impl AsRef<Path>) {
        let path_ref = path.as_ref();
        let path_str = path_ref.to_string_lossy();

        let url = url::Url::parse(&path_str)
            .or_else(|_| url::Url::from_file_path(path_ref))
            .unwrap_or_else(|_| {
                elog!(
                    &format!("Failed to parse path: {}", path_ref.display()).to_string(),
                    "gui-player"
                );
                return url::Url::parse("about:blank").unwrap();
            });

        let video = match Video::new(&url) {
            Ok(v) => v,
            Err(err) => {
                elog!(
                    &format!("Failed to load video: {:?}", err).to_string(),
                    "gui-player"
                );
                return;
            }
        };

        self.video = Some(video);
        self.position = 0.0;
        self.dragging = false;
    }

    pub fn update(&mut self, message: Message) {
        if let Some(video) = &mut self.video {
            match message {
                Message::Event(event) => match event {
                    Event::Keyboard(keyboard::Event::KeyPressed {
                        key: keyboard::Key::Named(key::Named::Space),
                        ..
                    }) => {
                        video.set_paused(!video.paused());
                    }
                    Event::Keyboard(keyboard::Event::KeyPressed {
                        key: keyboard::Key::Character(c),
                        ..
                    }) if c == "r" || c == "R" => video.set_looping(!video.looping()),
                    Event::Keyboard(keyboard::Event::KeyPressed {
                        key: keyboard::Key::Named(key::Named::ArrowRight),
                        repeat: false,
                        ..
                    }) => {
                        let now = Instant::now();

                        if let Some(last_press) = self.last_r_press {
                            if now.duration_since(last_press) <= app_static().double_tap_threshold {
                                self.last_r_press = None;

                                video
                                    .seek(Duration::from_secs_f64(self.position + 5.0), false)
                                    .expect("seek failed");
                            }
                        }

                        self.last_r_press = Some(now);
                    }
                    Event::Keyboard(keyboard::Event::KeyPressed {
                        key: keyboard::Key::Named(key::Named::ArrowLeft),
                        repeat: false,
                        ..
                    }) => {
                        let now = Instant::now();

                        if let Some(last_press) = self.last_r_press {
                            if now.duration_since(last_press) <= app_static().double_tap_threshold {
                                self.last_r_press = None;

                                video
                                    .seek(Duration::from_secs_f64(self.position - 5.0), false)
                                    .expect("seek failed");
                            }
                        }

                        self.last_r_press = Some(now);
                    }
                    _ => {}
                },
                Message::EndOfStream => {
                    blog!("end of stream", "gui-player");
                }
                Message::NewFrame => {
                    if !self.dragging {
                        self.position = video.position().as_secs_f64();
                    }
                }
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        match &self.video {
            Some(video) => Column::new()
                .push(
                    Container::new(
                        VideoPlayer::new(video)
                            .width(iced::Length::Fill)
                            .height(iced::Length::Fill)
                            .content_fit(iced::ContentFit::Contain)
                            .on_end_of_stream(Message::EndOfStream)
                            .on_new_frame(Message::NewFrame),
                    )
                    .align_x(iced::Alignment::Center)
                    .align_y(iced::Alignment::Center)
                    .width(iced::Length::Fill)
                    .height(iced::Length::Fill),
                )
                .into(),

            None => Text::new("No video loaded").into(),
        }
    }
}
