use iced::font::{Family, Stretch, Style, Weight};
use iced::{Font, Task, font};
use std::borrow::Cow;

pub const NERD_FONT: Font = Font {
    family: Family::Name("0xProto Nerd Font"),
    weight: Weight::Bold,
    stretch: Stretch::Normal,
    style: Style::Normal,
};

pub const NERD_FONT_BYTES: &[u8] = include_bytes!("../assets/fonts/0xProtoNerdFont-Bold.ttf");

pub fn load() -> Task<Result<(), font::Error>> {
    font::load(Cow::Borrowed(NERD_FONT_BYTES))
}
