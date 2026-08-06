use iced::{
    Element,
    widget::{Button, Column, Container, Row, Slider, Text},
};
use iced_video_player::{Video, VideoPlayer};
use std::{path::Path, time::Duration};

use crate::app::{blog, elog};

#[derive(Clone, Debug)]
pub enum Message {
    TogglePause,
    ToggleLoop,
    Seek(f64),
    SeekRelease,
    EndOfStream,
    NewFrame,
}

pub struct PlayerApp {
    video: Option<Video>,
    position: f64,
    dragging: bool,
}

impl PlayerApp {
    pub fn new() -> Self {
        Self {
            video: None,
            position: 0.0,
            dragging: false,
        }
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
                Message::TogglePause => {
                    video.set_paused(!video.paused());
                }
                Message::ToggleLoop => {
                    video.set_looping(!video.looping());
                }
                Message::Seek(secs) => {
                    self.dragging = true;
                    video.set_paused(true);
                    self.position = secs;
                }
                Message::SeekRelease => {
                    self.dragging = false;
                    video
                        .seek(Duration::from_secs_f64(self.position), false)
                        .expect("seek");
                    video.set_paused(false);
                }
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
                .push(
                    Container::new(
                        Slider::new(
                            0.0..=video.duration().as_secs_f64(),
                            self.position,
                            Message::Seek,
                        )
                        .step(0.1)
                        .on_release(Message::SeekRelease),
                    )
                    .padding(iced::Padding::new(5.0).left(10.0).right(10.0)),
                )
                .push(
                    Row::new()
                        .spacing(5)
                        .align_y(iced::alignment::Vertical::Center)
                        .padding(iced::Padding::new(10.0).top(0.0))
                        .push(
                            Button::new(Text::new(if video.paused() { "Play" } else { "Pause" }))
                                .width(80.0)
                                .on_press(Message::TogglePause),
                        )
                        .push(
                            Button::new(Text::new(if video.looping() {
                                "Disable Loop"
                            } else {
                                "Enable Loop"
                            }))
                            .width(120.0)
                            .on_press(Message::ToggleLoop),
                        )
                        .push(
                            Text::new(format!(
                                "{}:{:02}s / {}:{:02}s",
                                self.position as u64 / 60,
                                self.position as u64 % 60,
                                video.duration().as_secs() / 60,
                                video.duration().as_secs() % 60,
                            ))
                            .width(iced::Length::Fill)
                            .align_x(iced::alignment::Horizontal::Right),
                        ),
                )
                .into(),

            None => Text::new("No video loaded").into(),
        }
    }
}
