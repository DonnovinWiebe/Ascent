#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
use std::{io::{Error, ErrorKind, Result}, process::Child};
#[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
use std::process::Command;
use std::path::PathBuf;
use schrod::Schrod::{self, Pass, Fail};
use crate::data::{save_data::SaveDataType, tooling::Platforms};



// paths
/// The environment variable that can be used to specify the data directory in the server environment.
pub const ENV_VAR_DATA_DIRECTORY: &'static str = "ASCENT_DATA_DIR";

/// Gets the path to a given data file.
#[must_use]
pub fn path_to_file(save_data_type: SaveDataType, platform: Platforms) -> Schrod<PathBuf> {
    match platform {
        Platforms::Server => {
            // folder
            let folder_path = PathBuf::from(std::env::var(ENV_VAR_DATA_DIRECTORY).unwrap_or_else(|_| "./data".into()));
            let folder_creation_result = Schrod::from_result(std::fs::create_dir_all(&folder_path), "Failed to get create directory!", "paths::path_to_file()");
            if folder_creation_result.is_fail() {
                return folder_creation_result
                    .convert("paths::path_to_file()")
                    .fail("Failed to get file path.", "paths::path_to_file()")
            }
            
            // returning the file path
            Pass(join_file(folder_path, save_data_type))
        }
        
        Platforms::Desktop => {
            // folder
            let exe_path_result = Schrod::from_result(std::env::current_exe(), "Failed to get the executable directory!", "paths::path_to_file()");
            if exe_path_result.is_fail() {
                return exe_path_result
                    .convert("paths::path_to_file()")
                    .fail("Failed to get file path.", "paths::path_to_file()")
            }
            let exe_path = exe_path_result.wont_fail("This is past an is_fail() guard clause.", "paths::path_to_file()");

            // gets the folder path
            let mut folder_path = PathBuf::new();
            let mut folder_is_set = false;
            
            // gets the folder path when running as a .app bundle in MacOS
            #[cfg(target_os = "macos")]
            {
                if is_running_as_app_bundle(&exe_path) {
                    // the os based execution path
                    let base_result = Schrod::from_option(dirs::data_dir(), "Failed to fetch the OS data directory!", "paths::path_to_file()");
                    if base_result.is_fail() {
                        return base_result
                            .convert("paths::path_to_file()")
                            .fail("Failed to get file path.", "paths::path_to_file()")
                    }
                    let base = base_result.wont_fail("This is past an is_fail() guard clause.", "paths::path_to_file()");
        
                    // joins the base os path to an app specific folder
                    let save_location_path = base.join("Ascent").join("data");
                    let location_creation_result = Schrod::from_result(std::fs::create_dir_all(save_location_path.clone()), "Failed to create data dirctory!", "paths::path_to_file()");
                    if location_creation_result.is_fail() {
                        return location_creation_result
                            .convert("paths::path_to_file()")
                            .fail("Failed to get file path.", "paths::path_to_file()")
                    }

                    // sets the folder path
                    folder_path = save_location_path;
                    folder_is_set = true;
                }
            }

            // gets the folder path in regular cases
            if !folder_is_set {
                // upstream path
                let upstream_path_result = Schrod::from_option(exe_path.parent(), "Failed to get parent directory of the executable!", "paths::path_to_file()");
                if upstream_path_result.is_fail() {
                    return upstream_path_result
                        .convert("paths::path_to_file()")
                        .fail("Failed to get file path.", "paths::path_to_file()")
                }
                let upstream_path = upstream_path_result.wont_fail("This is past an is_fail() guard clause.", "paths::path_to_file()");
                
                // save location path
                let save_location_path = upstream_path.join("data");
                let location_creation_result = Schrod::from_result(std::fs::create_dir_all(save_location_path.clone()), "Failed to create data diretory!", "paths::path_to_file()");
                if location_creation_result.is_fail() {
                    return location_creation_result
                        .convert("paths::path_to_file()")
                        .fail("Failed to get file path.", "paths::path_to_file()")
                }

                // sets the folder path
                folder_path = save_location_path;
            }
            
            // returning the file path
            Pass(join_file(folder_path, save_data_type))
        }
        
        Platforms::Mobile => {
            todo!()
        }
    }
}

/// Checks if the path to the given file exists.
#[must_use]
pub fn does_file_exist(save_data_type: SaveDataType, platform: Platforms) -> bool {
    match path_to_file(save_data_type, platform) {
        Pass(path) => path.exists(),
        Fail(_) => false,
    }
}

/// Joins the right filename to a folder `Path` depending on the `SaveDataType`.
#[must_use]
fn join_file(folder_path: PathBuf, file: SaveDataType) -> PathBuf {
    match file {
        SaveDataType::Preferences => { folder_path.join("preferences.json") },
        SaveDataType::BankData => { folder_path.join("bank_data.json") },
        SaveDataType::BankDataBackup => {
            let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
            let filename = format!("backup_{timestamp}.json");
            folder_path.join("backups").join(filename)
        },
        SaveDataType::SyncData => { folder_path.join("sync.json") },
        SaveDataType::AuthData => { folder_path.join("auth.json") },
    }
}



// tools
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
pub fn open_data_location_file_explorer(platform: Platforms) -> Schrod<()> {
    match platform {
        Platforms::Server => {
            return Schrod::new_fail("Cannot open data location on the server.", "paths::open_data_location_window()");
        }
        
        Platforms::Desktop => {
            // gets the save data file path to work with
            let save_path_result = path_to_file(SaveDataType::BankData, platform);
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
        
        Platforms::Mobile => {
            return Schrod::new_fail("Cannot open data location on mobile.", "paths::open_data_location_window()");
        }
    }
}