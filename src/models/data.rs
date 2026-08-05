use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Payload {
    pub header: String,
    pub device_id: String,
    pub token: String,
    pub msg_type: String,
    pub msg_data: String,
    pub msg_params: Option<Vec<String>>,
    pub timestamp: u64,
}
impl Payload {
    pub fn get_param(&self, index: usize) -> Option<&str> {
        self.msg_params
            .as_ref()
            .and_then(|v| v.get(index))
            .map(|s| s.as_str())
    }
}
