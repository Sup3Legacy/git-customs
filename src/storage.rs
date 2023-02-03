use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{env, fs};

// path to ~/.config can be retreived from $XDG_CONFIG_HOME
// env::var("...")

#[derive(Serialize, Deserialize)]
pub struct Identity {
    name: String,
    email: String,
    gpg: Option<String>,
    host: Option<String>,
    directory: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct File {
    identities: Vec<Identity>,
}

pub enum ConfigFileError {
    PathDoesntExist,
    FileNotFound,
    ParseError,
}

impl File {
    pub fn read() -> Result<Self, ConfigFileError> {
        let config_path =
            env::var("$XDG_CONFIG_HOME").map_err(|_| ConfigFileError::PathDoesntExist)?;

        let file_path = Path::new(&config_path).join("gitcustoms.toml");

        let config_raw =
            fs::read_to_string(file_path).map_err(|_| ConfigFileError::FileNotFound)?;

        toml::from_str(&config_raw).map_err(|_| ConfigFileError::ParseError)
    }
}
