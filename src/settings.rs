extern crate confy;

use serde::{Serialize, Deserialize};

lazy_static! {
    pub static ref CONFIG: Settings = Settings::new().expect("config can't be loaded");
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Discord {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Messages {
    pub help: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub discord: Discord,
    pub messages: Messages,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            discord: Discord {
                token: "Unknown".to_string(),
            },
            messages: Messages {
                help: "Unknown help message".to_string(),
            }
        }
    }
}

impl Settings {
    pub fn new() -> Result<Self, confy::ConfyError> {
        confy::load("rustbot")
    }
}