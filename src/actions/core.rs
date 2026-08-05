use std::sync::OnceLock;
use std::sync::mpsc::{self, Sender};
use std::time::Duration;

use crate::actions::audio;
use crate::app::{blog, elog, runtime, wlog};
use crate::models::data::Payload;
use crate::models::{
    config::{AppConfig, CONFIG},
    data, queue,
};

static WORKER_TX: OnceLock<Sender<(Payload, &AppConfig, String)>> = OnceLock::new();

fn get_worker() -> &'static Sender<(Payload, &'static AppConfig, String)> {
    WORKER_TX.get_or_init(|| {
        let (tx, rx) = mpsc::channel::<(Payload, &AppConfig, String)>();
        let rt = runtime::get_runtime();
        rt.spawn(async move {
            while let Ok(value) = rx.recv() {
                rt.spawn(process(value.0, &value.1, value.2));
            }
        });
        tx
    })
}

async fn process(info: data::Payload, config: &AppConfig, host: String) {
    if info.msg_type == "TXT" {
        tokio::time::sleep(Duration::from_millis(config.delay_ms)).await;
        blog!(&format!("Received TXT message : {} from {}", info.msg_data, host).to_string());
    } else if info.msg_type == "AUD" {
        tokio::time::sleep(Duration::from_millis(config.delay_ms)).await;
        blog!(&format!("Received AUD request from {}", host).to_string());
        match audio::play(info.msg_data.to_string(), info.timestamp).await {
            Ok(()) => {}
            Err(e) => {
                elog!(&e.to_string());
            }
        }
    }
}

fn parse(msg: (String, String)) -> Result<(), serde_json::Error> {
    let info: data::Payload = serde_json::from_str(&msg.0)?;
    let config = CONFIG.get_or_init(|| AppConfig::gen_sample());

    if info.header != config.header {
        return Ok(());
    }

    if info.device_id != config.id {
        blog!("Detected Passerby", "core");
        return Ok(());
    }

    if info.token != config.token {
        elog!("Invalid token", "core");
        return Ok(());
    }

    if config.one_at_a_time {
        let tx = get_worker();
        let _ = tx.send((info.clone(), config, msg.1));
    } else {
        let rt = runtime::get_runtime();
        rt.spawn(process(info.clone(), config, msg.1));
    }

    Ok(())
}

pub fn listen() {
    while let Some(msg) = queue::pop_message() {
        if let Err(err) = parse(msg) {
            elog!(
                &format!("Failed to parse first message: {}", err).to_string(),
                "core"
            );
        };
    }

    wlog!("Queue channel closed. Exiting listener.", "core");
}
