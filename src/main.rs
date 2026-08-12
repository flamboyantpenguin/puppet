mod actions;
mod app;
mod gui;
mod misc;
mod models;
mod network;

use gui::App;

use crate::{app::blog, gui::NERD_FONT};

#[cfg(target_os = "windows")]
use crate::misc::win;

pub fn main() -> iced::Result {
    blog!("Starting app, expect config window...");
    #[cfg(target_os = "windows")]
    win::setup_gstreamer();

    app::controller::init();

    iced::application(App::new, App::update, App::view)
        .subscription(App::subscription)
        .title(App::title)
        .theme(App::theme)
        .default_font(NERD_FONT)
        .run()
}
