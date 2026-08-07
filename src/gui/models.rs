#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub(super) struct Config {
    pub header: String,
    pub id: String,
    pub delay_ms: String,
    pub token: String,
    pub port: String,
    pub one_at_a_time: bool,
    pub time_hash_token: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            header: "!Puppet93".into(),
            id: "0".into(),
            delay_ms: "0".into(),
            token: "MeowMeowMeow".into(),
            port: "8888".into(),
            one_at_a_time: false,
            time_hash_token: false,
        }
    }
}
