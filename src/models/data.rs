use serde::Deserialize;
use serde_json::Value;

#[derive(Clone, Deserialize, Debug)]
pub struct Payload {
    pub header: String,
    pub device_id: String,
    pub token: String,
    pub msg_type: String,
    pub msg_data: String,
    pub msg_params: Option<Value>,
    pub timestamp: u64,
}
impl Payload {
    pub fn get_param(&self, key: &str) -> Option<&Value> {
        self.msg_params.as_ref()?.get(key)
    }
}
