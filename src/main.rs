mod actions;
mod app;
mod gui;
mod models;
mod network;

use gui::App;

pub fn main() -> iced::Result {
    app::controller::init();

    iced::application(App::new, App::update, App::view)
        .subscription(App::subscription)
        .title(App::title)
        .theme(App::theme)
        .run()
}
