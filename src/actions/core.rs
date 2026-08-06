use std::sync::OnceLock;
use std::sync::mpsc::{self, Sender};

use chrono::{Duration, Utc};
use chrono_humanize::{Accuracy, HumanTime, Tense};

use crate::actions::audio;
use crate::app::controller::{GuiEvent, send_gui};
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
    let target_ms = info.timestamp * 1000;

    let now_ms = Utc::now().timestamp_millis() as u64;

    if target_ms != 0 && target_ms < now_ms {
        wlog!(
            &format!(
                "Received expired message from {}. Messaged supposed to run before {}",
                host,
                HumanTime::from(Duration::milliseconds((now_ms - target_ms) as i64))
                    .to_text_en(Accuracy::Precise, Tense::Present)
            )
            .to_string()
        );
        return;
    } else if target_ms >= now_ms {
        blog!(
            &format!("Received message for the future. Waiting...").to_string(),
            "core"
        );
        tokio::time::sleep(std::time::Duration::from_millis(target_ms - now_ms)).await;
    }

    // Now waiting until the predefined delay
    tokio::time::sleep(std::time::Duration::from_millis(config.delay_ms)).await;

    if info.msg_type == "TXT" {
        tokio::time::sleep(std::time::Duration::from_millis(config.delay_ms)).await;
        blog!(
            &format!("TXT message : {} from {}", info.msg_data, host).to_string(),
            "core"
        );
    } else if info.msg_type == "AUD" {
        tokio::time::sleep(std::time::Duration::from_millis(config.delay_ms)).await;
        let mut time_s = 0;
        if let Some(time) = info.get_param(0) {
            time_s = time.parse::<humantime::Duration>().unwrap().as_secs();
            blog!(
                &format!("Playing AUD request from {} for {}", host, time).to_string(),
                "core"
            );
        } else {
            blog!(
                &format!("Playing AUD request from {} till end", host).to_string(),
                "core"
            );
        }
        match audio::play(info.msg_data.to_string(), time_s).await {
            Ok(()) => {}
            Err(e) => {
                elog!(&e.to_string());
            }
        }
    } else if info.msg_type == "VID" {
        tokio::time::sleep(std::time::Duration::from_millis(config.delay_ms)).await;
        if let Some(time) = info.get_param(0) {
            blog!(
                &format!("Playing VID request from {} for {}", host, time).to_string(),
                "core"
            );
        } else {
            blog!(
                &format!("Playing VID request from {} till end", host).to_string(),
                "core"
            );
        }
        send_gui(GuiEvent::LoadVideo(info.msg_data));
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
