use nexus::data::{paths, save_data::{SaveDataType, SaveData}, tooling::Platforms};
use rand::random_range;
use schrod::Schrod::{Fail, Pass};

pub fn validate_token() {
    if !paths::does_file_exist(SaveDataType::AuthData, Platforms::Server) {
        let new_token = generate_token();
        let auth_data = SaveData::AuthData { token: new_token.clone() };
        let save_result = auth_data.save(SaveDataType::AuthData, Platforms::Server);
        match save_result {
            Pass(_) => tracing::info!("New token generated: {}", display_token(&new_token)),
            Fail(_) => {
                for er in save_result.results() { tracing::error!(er) }
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
            if i % 4 == 4 {
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

fn normalize_token(dirty_token: String) -> String {
    dirty_token
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_uppercase()
}