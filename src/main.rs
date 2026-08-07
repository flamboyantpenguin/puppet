mod actions;
mod app;
mod gui;
mod models;
mod network;

use gui::App;

use crate::gui::NERD_FONT;

pub fn main() -> iced::Result {
    app::controller::init();

    iced::application(App::new, App::update, App::view)
        .subscription(App::subscription)
        .title(App::title)
        .theme(App::theme)
        .default_font(NERD_FONT)
        .run()
}
