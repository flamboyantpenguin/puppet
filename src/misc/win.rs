#![cfg(target_os = "windows")]
use std::os::windows::ffi::OsStrExt;
use std::{env, path::PathBuf};
use windows::Win32::System::LibraryLoader::{
    AddDllDirectory, LOAD_LIBRARY_SEARCH_DEFAULT_DIRS, LOAD_LIBRARY_SEARCH_USER_DIRS,
    SetDefaultDllDirectories,
};

use crate::app::{blog, elog};

pub fn setup_gstreamer() {
    blog!("Setting up gstreamer paths...");
    let gstreamer_bin = env::var("PROGRAMFILES")
        .ok()
        .map(|program_files| {
            PathBuf::from(program_files)
                .join("gstreamer")
                .join("1.0")
                .join("msvc_x86_64")
                .join("bin")
        })
        .filter(|path| path.is_dir())
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

    let mut paths = vec![gstreamer_bin.clone()];

    if let Some(existing_path) = env::var_os("PATH") {
        paths.extend(env::split_paths(&existing_path));
    }

    let new_path = env::join_paths(paths).expect("Failed to construct PATH");

    let wide_path: Vec<u16> = gstreamer_bin
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    unsafe {
        SetDefaultDllDirectories(LOAD_LIBRARY_SEARCH_DEFAULT_DIRS | LOAD_LIBRARY_SEARCH_USER_DIRS)
            .expect("Failed to configure DLL search path");

        let cookie = AddDllDirectory(windows::core::PCWSTR(wide_path.as_ptr()));

        assert!(!cookie.is_null(), "Failed to add GStreamer DLL directory");

        env::set_var("PATH", new_path);
    }
}
