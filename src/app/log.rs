use chrono::Local;
use owo_colors::{OwoColorize, Stream};

macro_rules! elog {
    ($msg:expr) => {
        $crate::app::log::eprintln($msg, None);
    };
    ($msg:expr, $who:expr) => {
        $crate::app::log::eprintln($msg, Some($who));
    };
}

macro_rules! blog {
    ($msg:expr) => {
        $crate::app::log::bprintln($msg, None);
    };
    ($msg:expr, $who:expr) => {
        $crate::app::log::bprintln($msg, Some($who));
    };
}

macro_rules! wlog {
    ($msg:expr) => {
        $crate::app::log::wprintln($msg, None);
    };
    ($msg:expr, $who:expr) => {
        $crate::app::log::wprintln($msg, Some($who));
    };
}

macro_rules! glog {
    ($msg:expr) => {
        $crate::app::log::gprintln($msg, None);
    };
    ($msg:expr, $who:expr) => {
        $crate::app::log::gprintln($msg, Some($who));
    };
}

pub(crate) use blog;
pub(crate) use elog;
pub(crate) use glog;
pub(crate) use wlog;

pub fn eprintln(msg: &str, who: Option<&str>) {
    let log_message = format!(
        "[ERROR] {} :: {} >> {}",
        Local::now().format("%Y/%m/%d|%H:%M:%S"),
        who.unwrap_or("app"),
        msg
    );

    eprintln!(
        "{}",
        log_message.if_supports_color(Stream::Stderr, |text| text.red())
    );
}

pub fn wprintln(msg: &str, who: Option<&str>) {
    let log_message = format!(
        "[WARN] {} :: {} >> {}",
        Local::now().format("%Y/%m/%d|%H:%M:%S"),
        who.unwrap_or("app"),
        msg
    );

    eprintln!(
        "{}",
        log_message.if_supports_color(Stream::Stderr, |text| text.yellow())
    );
}

pub fn bprintln(msg: &str, who: Option<&str>) {
    let log_message = format!(
        "[INFO] {} :: {} >> {}",
        Local::now().format("%Y/%m/%d|%H:%M:%S"),
        who.unwrap_or("app"),
        msg
    );

    println!(
        "{}",
        log_message.if_supports_color(Stream::Stderr, |text| text.blue())
    );
}

pub fn gprintln(msg: &str, who: Option<&str>) {
    let log_message = format!(
        "[DULCET] {} :: {} >> {}",
        Local::now().format("%Y/%m/%d|%H:%M:%S"),
        who.unwrap_or("app"),
        msg
    );

    println!(
        "{}",
        log_message.if_supports_color(Stream::Stderr, |text| text.green())
    );
}
