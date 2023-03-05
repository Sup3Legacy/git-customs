use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::{env, fs};

// path to ~/.config can be retreived from $XDG_CONFIG_HOME
// env::var("...")

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    email: Option<String>,
    gpg: Option<String>,
    // Switch to an (optional) array of hosts and directory
    host: Option<Vec<String>>,
    directory: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Commit {
    #[serde(rename = "gpgSign")]
    gpg_sign: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pull {
    rebase: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Identity {
    // Switch to an (optional) array of hosts and directory
    host: Option<Vec<String>>,
    directory: Option<Vec<String>>,

    user: Option<User>,
    commit: Option<Commit>,
    pull: Option<Pull>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    identities: HashMap<String, Identity>,
}

#[derive(Debug)]
pub enum ConfigFileError {
    PathDoesntExist,
    FileNotFound,
    ParseError,
}

impl File {
    pub fn read() -> Result<Self, ConfigFileError> {
        let config_path =
            env::var("XDG_CONFIG_HOME").map_err(|_| ConfigFileError::PathDoesntExist)?;

        let file_path = Path::new(&config_path)
            .join("gitcustoms.toml")
            .canonicalize()
            .map_err(|_| ConfigFileError::PathDoesntExist)?;

        let config_raw =
            fs::read_to_string(file_path).map_err(|_| ConfigFileError::FileNotFound)?;

        let truc: Result<HashMap<String, Identity>, _> = toml::from_str(&config_raw);

        match truc {
            Err(e) => {
                println!("{:#?}", e);
                Err(ConfigFileError::ParseError)
            }
            Ok(u) => Ok(Self { identities: u }),
        }
    }
}
