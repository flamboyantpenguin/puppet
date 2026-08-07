use iced::widget::container;
use iced::{Element, Length};
use iced_gif::widget::gif;

use crate::gui::app::Message;
use crate::gui::assets::IDLE_VID;

#[derive(Default)]
pub struct Idle {
    frames: Option<gif::Frames>,
}

impl Idle {
    pub fn new() -> Self {
        let frames =
            gif::Frames::from_bytes(IDLE_VID.to_vec()).expect("Failed to decode embedded GIF");

        Idle {
            frames: Some(frames),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        if let Some(frames) = self.frames.as_ref() {
            container(gif(frames))
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into()
        } else {
            container(iced::widget::text("GIF state is None!"))
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into()
        }
    }
}
