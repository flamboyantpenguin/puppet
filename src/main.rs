mod actions;
mod models;
mod network;

use std::thread;

use actions::core;
use network::udp;

fn main() {
    thread::spawn(|| {
        udp::listen().expect("Failed");
    });
    thread::spawn(|| core::listen());
    loop {
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}
