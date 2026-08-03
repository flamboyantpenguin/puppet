use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Payload {
    pub device_id: String,
    pub msg_type: String,
    pub msg_data: String,
    pub timestamp: u64,
}
