extern crate confy;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub token: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            token: "Unknown".to_string()
        }
    }
}

impl Settings {
    pub fn new() -> Result<Self, confy::ConfyError> {
        confy::load("rustbot")
    }
}