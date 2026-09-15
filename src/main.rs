#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod actions;
mod app;
mod gui;
mod models;
mod network;

use gui::App;
use iced::window;

use crate::{app::blog, gui::NERD_FONT};

pub fn main() -> iced::Result {
    blog!("Starting app, expect config window...");

    app::controller::init();

    iced::application(App::new, App::update, App::view)
        .subscription(App::subscription)
        .title(App::title)
        .theme(App::theme)
        .window(window::Settings {
            fullscreen: true,
            ..Default::default()
        })
        .default_font(NERD_FONT)
        .run()
}
