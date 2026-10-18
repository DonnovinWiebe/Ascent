#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
use std::{io::{Error, ErrorKind, Result}, process::Child};
#[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
use std::process::Command;
use std::path::PathBuf;
use schrod::Schrod::{self, Pass, Fail};


/// Returns the `Path` to the save location and creates it if it doesn't exist.
#[must_use]
pub fn save_path() -> Schrod<PathBuf> {
    // executable path
    let exe_path_result = Schrod::from_result(std::env::current_exe(), "Failed to fetch the executable directory.", "paths::save_path()");
    if exe_path_result.is_fail() {
        return exe_path_result
            .convert("paths::save_path()")
            .fail("Failed to save.", "paths::save_path()")
    }
    let exe_path = exe_path_result.wont_fail("This is past an is_fail() guard clause.", "paths::save_path()");

    // modifies the path when running as a .app bundle in MacOS
    #[cfg(target_os = "macos")]
    {
        if is_running_as_app_bundle(&exe_path) {
            // the os based execution path

            use schrod::Schrod::Pass;
            let base_result = Schrod::from_option(dirs::data_dir(), "Failed to fetch the OS data directory.", "paths::save_path()");
            if base_result.is_fail() {
                return base_result
                    .convert("paths::save_path()")
                    .fail("Failed to save.", "paths::save_path()")
            }
            let base = base_result.wont_fail("This is past an is_fail() guard clause.", "paths::save_path()");

            // joins the base os path to an app specific folder
            let save_location_path = base.join("Ascent").join("save_data");
            let location_creation_result = Schrod::from_result(std::fs::create_dir_all(save_location_path.clone()), "Failed to create save data location.", "paths::save_path()");
            if location_creation_result.is_fail() {
                return location_creation_result
                    .convert("paths::save_path()")
                    .fail("Failed to save.", "paths::save_path()")
            }

            // returns the final save data path
            return Pass(save_location_path.join("data.json"));
        }
    }
        
    // upstream path
    let upstream_path_result = Schrod::from_option(exe_path.parent(), "Failed to get parent directory of the executable.", "paths::save_path()");
    if upstream_path_result.is_fail() {
        return upstream_path_result
            .convert("paths::save_path()")
            .fail("Failed to save.", "paths::save_path()")
    }
    let upstream_path = upstream_path_result.wont_fail("This is past an is_fail() guard clause.", "paths::save_path()");
    
    // save location path
    let save_location_path = upstream_path.join("save_data");
    let location_creation_result = Schrod::from_result(std::fs::create_dir_all(save_location_path.clone()), "Failed to create save data location.", "paths::save_path()");
    if location_creation_result.is_fail() {
        return location_creation_result
            .convert("paths::save_path()")
            .fail("Failed to save.", "paths::save_path()")
    }
    
    // save path
    let save_path = save_location_path.join("data.json");
    
    // returning the save path
    Pass(save_path)
}

/// Returns the `Path` to the backup location and creates it if it doesn't exist.
#[must_use]
pub fn backup_path() -> Schrod<PathBuf> {
    // executable path
    let exe_path_result = Schrod::from_result(std::env::current_exe(), "Failed to fetch the executable directory.", "paths::backup_path()");
    if exe_path_result.is_fail() {
        return exe_path_result
            .convert("paths::backup_path()")
            .fail("Failed to create backup.", "paths::backup_path()")
    }
    let exe_path = exe_path_result.wont_fail("This is past an is_fail() guard clause.", "paths::backup_path()");
    
    // modifies the path when running as a .app bundle in MacOS
    #[cfg(target_os = "macos")]
    {
        if is_running_as_app_bundle(&exe_path) {
            // the os based execution path

            use schrod::Schrod::Pass;
            let base_result = Schrod::from_option(dirs::data_dir(), "Failed to fetch the OS data directory.", "paths::backup_path()");
            if base_result.is_fail() {
                return base_result
                    .convert("paths::backup_path()")
                    .fail("Failed to create backup.", "paths::backup_path()")
            }
            let base = base_result.wont_fail("This is past an is_fail() guard clause.", "paths::backup_path()");

            // joins the base os path to an app specific folder
            let backup_location_path = base.join("Ascent").join("backups");
            let location_creation_result = Schrod::from_result(std::fs::create_dir_all(backup_location_path.clone()), "Failed to create backup location.", "paths::backup_path()");
            if location_creation_result.is_fail() {
                return location_creation_result
                    .convert("paths::backup_path()")
                    .fail("Failed to create backup.", "paths::backup_path()")
            }

            // save path
            let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
            let filename = format!("backup_{timestamp}.json");
            let export_path = backup_location_path.join(filename);
            
            // returns the final save path
            return Pass(export_path);
        }
    }
    
    // upstream path
    let upstream_path_result = Schrod::from_option(exe_path.parent(), "Failed to get parent directory of the executable.", "paths::backup_path()");
    if upstream_path_result.is_fail() {
        return upstream_path_result
            .convert("paths::backup_path()")
            .fail("Failed to create backup.", "paths::backup_path()")
    }
    let upstream_path = upstream_path_result.wont_fail("This is past an is_fail() guard clause.", "paths::backup_path()");
    
    // save location path
    let backup_location_path = upstream_path.join("backups");
    let location_creation_result = Schrod::from_result(std::fs::create_dir_all(backup_location_path.clone()), "Failed to create backup location.", "paths::backup_path()");
    if location_creation_result.is_fail() {
        return location_creation_result
            .convert("paths::backup_path()")
            .fail("Failed to create backup.", "paths::backup_path()");
    }

    // save path
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
    let filename = format!("backup_{timestamp}.json");
    let export_path = backup_location_path.join(filename);
    
    // returning the save path
    Pass(export_path)
}

/// Checks if the save file exists.
#[must_use]
pub fn does_save_file_exist() -> bool {
    match save_path() {
        Pass(path) => path.exists(),
        Fail(_) => false,
    }
}

/// Checks if the app is running from inside a macOS .app bundle
#[must_use]
#[cfg(target_os = "macos")]
pub fn is_running_as_app_bundle(exe_path: &std::path::Path) -> bool {
    exe_path.ancestors().any(|p| {
        p.extension().is_some_and(|ext| ext == "app")
    })
}

/// Opens a system file explorer at the `App`'s data directory.
#[must_use]
pub fn open_data_location_file_explorer() -> Schrod<()> {
    // gets the save data file path to work with
    let save_path_result = save_path();
    if save_path_result.is_fail() {
        return save_path_result
            .convert("paths::open_data_location_window()")
            .fail("Failed to open data location.", "paths::open_data_location_window()")
    }
    let save_path = save_path_result.wont_fail("This is past an is_fail() guard clause.", "paths::open_data_location_window()");

    // gets the save folder that holds all data
    let save_folder_result = Schrod::from_option(save_path.parent(), "Failed to get parent directory of the save data.", "paths::reveal_save_location()");
    if save_folder_result.is_fail() {
        return save_folder_result
            .convert("paths::open_data_location_window()")
            .fail("Failed to open data location.", "paths::open_data_location_window()")
    }
    let save_folder = save_folder_result.wont_fail("This is past an is_fail() guard clause.", "paths::open_data_location_window()");
    
    // gets the parent folder that holds all data
    let home_directory_result = Schrod::from_option(save_folder.parent(), "Failed to get the data directory.", "paths::reveal_save_location()");
    if home_directory_result.is_fail() {
        return home_directory_result
            .convert("paths::open_data_location_window()")
            .fail("Failed to open data location.", "paths::open_data_location_window()")
    }
    let home_directory = home_directory_result.wont_fail("This is past an is_fail() guard clause.", "paths::open_data_location_window()");

    // opens the file explorer
    #[cfg(target_os = "macos")]
    let spawn_result = Command::new("open").arg(home_directory).spawn();
    #[cfg(target_os = "linux")]
    let spawn_result = Command::new("xdg-open").arg(home_directory).spawn();
    #[cfg(target_os = "windows")]
    let spawn_result = Command::new("explorer").arg(home_directory).spawn();

    // fails if this is (somehow) running on a different os
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    let spawn_result: Resultesult<Child> =
        Err(Error::new(ErrorKind::Unsupported, "Unsupported platform for opening file location"));

    // returns the result
    let result = Schrod::from_result(spawn_result, "Failed to open file explorer window.", "paths::open_data_location_window()");
    match result {
        Pass(_) => Pass(()),
        Fail(_) => {
            result
                .convert("paths::open_data_location_window()")
                .fail("Failed to open data location.", "paths::open_data_location_window()")
        }
    }
}