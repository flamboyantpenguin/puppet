use std::io::Result;
use std::net::UdpSocket;

use crate::app::{blog, elog};
use crate::models::config::{AppConfig, CONFIG};

use crate::models::queue::get_or_init_sender;

pub fn listen() -> Result<()> {
    let config = CONFIG.get_or_init(|| AppConfig::gen_sample());
    let broadcast_port = config.port;
    let socket = UdpSocket::bind(format!("0.0.0.0:{}", broadcast_port))?;
    blog!(
        &format!("Listening for UDP broadcasts on port {}...", broadcast_port).to_string(),
        "udp-listener"
    );

    let mut buffer = [0u8; 1024];

    loop {
        let (number_of_bytes, addr) = socket.recv_from(&mut buffer)?;

        let received_message = String::from_utf8_lossy(&buffer[..number_of_bytes]).to_string();

        let tx = get_or_init_sender();

        if tx.send((received_message, addr.to_string())).is_err() {
            elog!("Actions consumer dropped. Exiting loop.", "udp-listener");
            break;
        }
    }
    Ok(())
}
