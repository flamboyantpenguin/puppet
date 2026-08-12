#![cfg(target_os = "windows")]
use crate::app::{blog, elog};
use std::{env, path::PathBuf};

pub fn setup_gstreamer() {
    blog!("Setting up gstreamer paths...");
    let gstreamer_bin = {
        if let Ok(program_files) = env::var("PROGRAMFILES") {
            let path = PathBuf::from(program_files)
                .join("gstreamer")
                .join("1.0")
                .join("msvc_x86_64")
                .join("bin");

            if path.is_dir() { Some(path) } else { None }
        } else {
            None
        }
    }
    .or_else(|| {
        env::var("LOCALAPPDATA").ok().and_then(|local_app_data| {
            let path = PathBuf::from(local_app_data)
                .join("gstreamer")
                .join("1.0")
                .join("msvc_x86_64")
                .join("bin");

            path.is_dir().then_some(path)
        })
    })
    .or_else(|| {
        env::current_exe()
            .ok()
            .and_then(|exe| {
                exe.parent()
                    .map(|exe_dir| exe_dir.join("gstreamer").join("bin"))
            })
            .filter(|path| path.is_dir())
    });

    let Some(gstreamer_bin) = gstreamer_bin else {
        elog!("gstreamer not found in system. Check docs or contact support");
        panic!("gstreamer not found in system");
    };

    let mut paths = vec![gstreamer_bin];

    if let Some(existing_path) = env::var_os("PATH") {
        paths.extend(env::split_paths(&existing_path));
    }

    let new_path = env::join_paths(paths).expect("Failed to construct PATH");

    unsafe {
        env::set_var("PATH", new_path);
    }
}
