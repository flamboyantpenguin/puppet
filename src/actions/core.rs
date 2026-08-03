use std::sync::OnceLock;
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::Duration;

use crate::models::{
    config::{AppConfig, CONFIG},
    data, queue,
};

static WORKER_TX: OnceLock<Sender<data::Payload>> = OnceLock::new();

fn get_worker() -> &'static Sender<data::Payload> {
    WORKER_TX.get_or_init(|| {
        let (tx, rx) = mpsc::channel::<data::Payload>();
        thread::spawn(move || {
            let config = CONFIG.get_or_init(|| AppConfig::gen_sample());
            while let Ok(info) = rx.recv() {
                process(&info, config);
            }
        });
        tx
    })
}

fn process(info: &data::Payload, config: &AppConfig) {
    if info.msg_type == "TXT" {
        thread::sleep(Duration::from_millis(config.delay_ms));
        println!("{}", info.msg_data);
    }
}

fn parse(msg: &str) -> Result<(), serde_json::Error> {
    let info: data::Payload = serde_json::from_str(msg)?;
    let config = CONFIG.get_or_init(|| AppConfig::gen_sample());

    //let info: data::Payload = match serde_json::from_str(&msg) {
    //    Ok(info) => info,
    //    Err(err) => {
    //        return Err(err);
    //    }
    //};
    //

    if config.one_at_a_time {
        // Send to single background thread (strictly sequential)
        let tx = get_worker();
        let _ = tx.send(info);
    } else {
        // Concurrent behavior (spawn dedicated thread)
        thread::spawn(move || {
            process(&info, config);
        });
    }

    Ok(())
}

pub fn listen() {
    while let Some(msg) = queue::pop_message() {
        //match parse(&msg) {
        //    Ok(_) => {}
        //    Err(_) => {
        //        continue;
        //    }
        //};
        if let Err(err) = parse(&msg) {
            eprintln!("Failed to parse first message: {}", err);
        };
    }

    println!("Queue channel closed. Exiting listener.");
}
