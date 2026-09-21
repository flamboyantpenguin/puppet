use std::{sync::OnceLock, time::Duration};

pub struct AppStatic {
    pub app_name: &'static str,
    pub version: &'static str,
    pub github_url: &'static str,
    pub package_url: &'static str,
    pub puppeteer_url: &'static str,
    pub double_tap_threshold: Duration,
}

pub fn app_static() -> &'static AppStatic {
    static INSTANCE: OnceLock<AppStatic> = OnceLock::new();
    INSTANCE.get_or_init(|| AppStatic {
        app_name: "puppet",
        version: env!("CARGO_PKG_VERSION"),
        github_url: "https://github.com/flamboyantpenguin/puppet",
        package_url: "https://code.dawn.org.in/flamboyantpenguin/puppet/packages",
        puppeteer_url: "https://github.com/flamboyantpenguin/puppeteer",
        double_tap_threshold: Duration::from_millis(250),
    })
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AppConfig {
    pub header: String,
    pub device_id: String,
    pub delay_ms: u64,
    pub token: String,
    pub port: u64,
    pub one_at_a_time: bool,
    pub time_hash_token: bool,
    pub show_idle: bool,
}
impl AppConfig {
    pub fn default() -> AppConfig {
        return AppConfig {
            header: "!Puppet93".to_string(),
            device_id: "0".to_string(),
            delay_ms: 0,
            token: "MeowMeowMeow".to_string(),
            port: 8888,
            one_at_a_time: false,
            time_hash_token: false,
            show_idle: false,
        };
    }
}

pub static CONFIG: OnceLock<AppConfig> = OnceLock::new();

pub fn app_config() -> Option<&'static AppConfig> {
    CONFIG.get()
}
