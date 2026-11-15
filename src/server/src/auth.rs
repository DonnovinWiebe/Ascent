use nexus::data::{paths, save_data::{SaveData, SaveDataType::{self, AuthData}}, tooling::Platforms};
use rand::random_range;
use schrod::Schrod::{Fail, Pass};

pub fn validate_token() -> String {
    tracing::info!("Getting token...");
    if !paths::does_file_exist(SaveDataType::AuthData, Platforms::Server) {
        tracing::info!("Token not found.");
        tracing::info!("Generating new token...");
        let new_token = generate_token();
        let auth_data = SaveData::AuthData { token: new_token.clone() };
        let save_result = auth_data.save(SaveDataType::AuthData, Platforms::Server);
        match save_result {
            Pass(_) => {
                tracing::info!("New token generated: {}", display_token(&new_token));
                new_token
            }
            Fail(_) => {
                for er in save_result.results() { tracing::error!(er) }
                tracing::warn!("Using random token...");
                generate_token()
            }
        }
    }
    else {
        tracing::info!("Token found.");
        let auth_data_result = SaveData::load(SaveDataType::AuthData, Platforms::Server);
        match auth_data_result {
            Pass(auth_data) => {
                match auth_data {
                    SaveData::AuthData { token } => token,
                    _ => {
                        tracing::error!("Failed to load token!");
                        tracing::warn!("Using random token...");
                        generate_token()
                    }
                }
            }
            
            Fail(_) => {
                tracing::error!("Failed to load token!");
                tracing::warn!("Using random token...");
                generate_token()
            }
        }
    }
}

/// Generates a fresh token
fn generate_token() -> String {
    let allowed_chars: &[u8] = b"ABCDEFGHJKMNPQRSTUVWXYZ23456789";
    (0..16)
        .map(|_| allowed_chars[random_range(0..allowed_chars.len())] as char)
        .collect()
}

pub fn display_token(token: &str) -> String {
    // inserts dashes every 4 characters
    let mut i = 0;
    let mut display_token: String = token
        .chars()
        .map(|c| {
            i+= 1;
            if i % 4 == 0 {
                i = 0;
                format!("{}{}", c, "-")
            }
            else { c.to_string() }
        })
        .collect();

    // removes the last character if the last character is a dash
    let last_char = display_token.pop();
    if let Some(char) = last_char {
        if char.to_string() != "-".to_string() { display_token.push(char) }
    }

    // returns the displayable token
    display_token
}