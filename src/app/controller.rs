use std::sync::{OnceLock, mpsc};
use std::thread;

use crate::app::glog;
use crate::{actions::core, network::udp};

pub enum AppEvent {
    ConfigSaved,
}

static EVENTS: OnceLock<mpsc::Sender<AppEvent>> = OnceLock::new();

pub fn init() {
    let (tx, rx) = mpsc::channel();

    EVENTS.set(tx).unwrap();

    thread::spawn(move || {
        while let Ok(event) = rx.recv() {
            match event {
                AppEvent::ConfigSaved => {
                    glog!("Starting listeners...");

                    thread::spawn(|| {
                        udp::listen().unwrap();
                    });

                    thread::spawn(core::listen);
                }
            }
        }
    });
}

pub fn send(event: AppEvent) {
    if let Some(tx) = EVENTS.get() {
        let _ = tx.send(event);
    }
}
