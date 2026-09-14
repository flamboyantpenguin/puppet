use std::fs::File;

use crate::{
    app::{blog, elog, wlog},
    models::config::{AppConfig, CONFIG, app_config, app_static},
};

pub fn load_config() {
    let xdg_dirs = xdg::BaseDirectories::with_prefix(app_static().app_name);

    match xdg_dirs.get_config_file("prev.json") {
        Some(file_path) => {
            let Ok(file) = File::open(file_path) else {
                wlog!("Unable to open config, may not exist");
                return;
            };
            if let Ok(imported_data) = serde_json::from_reader::<_, AppConfig>(file) {
                CONFIG.set(imported_data).ok();
                blog!("Previous config loaded");
            }
        }
        None => {}
    }
}

pub fn save_config() {
    let xdg_dirs = xdg::BaseDirectories::with_prefix(app_static().app_name);

    let Ok(file_path) = xdg_dirs.place_config_file("prev.json") else {
        elog!("Unable to create config");
        return;
    };
    let Ok(file) = File::create(file_path) else {
        elog!("Unable to open config");
        return;
    };
    if let Err(_) = serde_json::to_writer_pretty(file, app_config()) {
        elog!("Unable to write config");
        return;
    }
}
