use std::sync::OnceLock;

pub struct AppConfig {
    pub header: String,
    pub id: String,
    pub delay_ms: u64,
    pub token: String,
    pub port: u64,
    pub one_at_a_time: bool,
    pub time_hash_token: bool,
}
impl AppConfig {
    pub fn gen_sample() -> AppConfig {
        return AppConfig {
            header: "!Puppet93".to_string(),
            id: "0".to_string(),
            delay_ms: 0,
            token: "MeowMeowMeow".to_string(),
            port: 8888,
            one_at_a_time: false,
            time_hash_token: false,
        };
    }
}

pub static CONFIG: OnceLock<AppConfig> = OnceLock::new();
