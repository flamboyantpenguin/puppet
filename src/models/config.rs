use std::sync::OnceLock;

pub struct AppStatic {
    pub github_url: &'static str,
    pub package_url: &'static str,
    pub puppeteer_url: &'static str,
}

pub fn app_static() -> &'static AppStatic {
    static INSTANCE: OnceLock<AppStatic> = OnceLock::new();
    INSTANCE.get_or_init(|| AppStatic {
        github_url: "https://github.com/flamboyantpenguin/puppet",
        package_url: "https://code.dawn.org.in/flamboyantpenguin/puppet/packages",
        puppeteer_url: "https://github.com/flamboyantpenguin/puppeteer",
    })
}

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
