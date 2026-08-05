use std::sync::Mutex;
use std::sync::OnceLock;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

static QUEUE_TX: OnceLock<Sender<(String, String)>> = OnceLock::new();
static QUEUE_RX: OnceLock<Mutex<Receiver<(String, String)>>> = OnceLock::new();

pub fn get_or_init_sender() -> Sender<(String, String)> {
    QUEUE_TX
        .get_or_init(|| {
            let (tx, rx) = mpsc::channel();
            let _ = QUEUE_RX.set(Mutex::new(rx));
            tx
        })
        .clone()
}

pub fn pop_message() -> Option<(String, String)> {
    let mutex = loop {
        if let Some(rx) = QUEUE_RX.get() {
            break rx;
        }
        thread::sleep(Duration::from_millis(10));
    };

    let rx_guard = mutex.lock().unwrap_or_else(|e| e.into_inner());
    rx_guard.recv().ok()
}
