use std::sync::{OnceLock, mpsc as std_mpsc};
use std::thread;
use tokio::sync::{Mutex, mpsc};

use crate::app::glog;
use crate::{actions::core, network::udp};

pub enum AppEvent {
    ConfigSaved,
}

#[derive(Debug, Clone)]
pub enum GuiEvent {
    LoadVideo(String),
}

static EVENTS: OnceLock<std_mpsc::Sender<AppEvent>> = OnceLock::new();

static GUI_SENDER: OnceLock<mpsc::UnboundedSender<GuiEvent>> = OnceLock::new();
static GUI_RECEIVER: OnceLock<tokio::sync::Mutex<mpsc::UnboundedReceiver<GuiEvent>>> =
    OnceLock::new();

pub fn init() {
    let (tx, rx) = std_mpsc::channel();
    let (g_tx, g_rx) = mpsc::unbounded_channel();

    EVENTS.set(tx).unwrap();
    GUI_SENDER.set(g_tx).unwrap();
    GUI_RECEIVER.set(Mutex::new(g_rx)).unwrap();

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

pub fn send_gui(event: GuiEvent) {
    if let Some(tx) = GUI_SENDER.get() {
        let _ = tx.send(event);
    }
}

pub fn gui_receiver() -> &'static tokio::sync::Mutex<mpsc::UnboundedReceiver<GuiEvent>> {
    GUI_RECEIVER.get().unwrap()
}
