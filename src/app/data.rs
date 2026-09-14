use directories::ProjectDirs;
use std::fs::{self, File};

use crate::{
    app::{blog, elog, wlog},
    models::config::{AppConfig, CONFIG, app_config, app_static},
};

pub fn load_config() {
    let Some(dirs) = ProjectDirs::from("", "", app_static().app_name) else {
        elog!("Unable to determine xdg directories");
        return;
    };

    let config_file = dirs.config_dir().join("prev.json");

    let Ok(file) = File::open(config_file) else {
        wlog!("Previous config not found, must be new run");
        return;
    };

    if let Ok(imported_data) = serde_json::from_reader::<_, AppConfig>(file) {
        CONFIG.set(imported_data).ok();
        blog!("Previous config loaded");
    } else {
        elog!("Invalid config. Corruption perhaps?");
    }
}

pub fn save_config() {
    let Some(dirs) = ProjectDirs::from("", "", app_static().app_name) else {
        elog!("Unable to determine config directory");
        return;
    };

    let config_dir = dirs.config_dir();

    if let Err(_) = fs::create_dir_all(config_dir) {
        elog!("Unable to create config directories");
        return;
    }

    let config_file = config_dir.join("prev.json");

    let Ok(file) = File::create(config_file) else {
        elog!("Unable to open/create config");
        return;
    };

    if let Err(_) = serde_json::to_writer_pretty(file, app_config()) {
        elog!("Unable to write config");
        return;
    }
}
