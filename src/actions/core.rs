use std::sync::OnceLock;
use std::sync::mpsc::{self, Sender};
use std::time::Duration;

use crate::actions::audio;
use crate::app::{print, runtime};
use crate::models::{
    config::{AppConfig, CONFIG},
    data, queue,
};

static WORKER_TX: OnceLock<Sender<data::Payload>> = OnceLock::new();

fn get_worker() -> &'static Sender<data::Payload> {
    WORKER_TX.get_or_init(|| {
        let (tx, rx) = mpsc::channel::<data::Payload>();
        let rt = runtime::get_runtime();
        rt.spawn(async move {
            let config = CONFIG.get_or_init(|| AppConfig::gen_sample());
            while let Ok(info) = rx.recv() {
                rt.spawn(process(info.clone(), config));
            }
        });
        tx
    })
}

async fn process(info: data::Payload, config: &AppConfig) {
    if info.msg_type == "TXT" {
        tokio::time::sleep(Duration::from_millis(config.delay_ms)).await;
        println!("{}", info.msg_data);
    } else if info.msg_type == "AUD" {
        tokio::time::sleep(Duration::from_millis(config.delay_ms)).await;
        match audio::play(info.msg_data.to_string(), info.timestamp).await {
            Ok(()) => {}
            Err(e) => {
                print::eprintln(&e.to_string());
            }
        }
    }
}

fn parse(msg: &str) -> Result<(), serde_json::Error> {
    let info: data::Payload = serde_json::from_str(msg)?;
    let config = CONFIG.get_or_init(|| AppConfig::gen_sample());

    if info.header != config.header {
        return Ok(());
    }

    if info.device_id != config.id {
        print::bprintln("Detected passerby");
        return Ok(());
    }

    if info.token != config.token {
        print::eprintln("Invalid Token!");
        return Ok(());
    }

    //let info: data::Payload = match serde_json::from_str(&msg) {
    //    Ok(info) => info,
    //    Err(err) => {
    //        return Err(err);
    //    }
    //};
    //

    if config.one_at_a_time {
        let tx = get_worker();
        let _ = tx.send(info);
    } else {
        let rt = runtime::get_runtime();
        rt.spawn(process(info.clone(), config));
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
            print::eprintln(&format!("Failed to parse first message: {}", err).to_string());
        };
    }

    print::bprintln("Queue channel closed. Exiting listener.");
}
