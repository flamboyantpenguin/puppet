use owo_colors::{OwoColorize, Stream};

pub fn eprintln(msg: &str) {
    eprintln!(
        "{}",
        msg.if_supports_color(Stream::Stdout, |text| text.red())
    );
}

pub fn bprintln(msg: &str) {
    println!(
        "{}",
        msg.if_supports_color(Stream::Stdout, |text| text.blue())
    );
}

pub fn gprintln(msg: &str) {
    println!(
        "{}",
        msg.if_supports_color(Stream::Stdout, |text| text.green())
    );
}
