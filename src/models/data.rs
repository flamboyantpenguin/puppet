use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Payload {
    pub header: String,
    pub device_id: String,
    pub token: String,
    pub msg_type: String,
    pub msg_data: String,
    pub timestamp: u64,
}
