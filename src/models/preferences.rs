use failure::Error;
use serenity::model::id::{ChannelId, GuildId};
use toml;

use std::env;
use std::fs::OpenOptions;
use std::io::Read;

use errors::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Preferences {
    pub token: String,
    timestamp_fmt: Option<String>,
    pub guild: Option<GuildId>,
    pub channel: Option<ChannelId>,
}

impl Preferences {
    pub fn load() -> Result<Preferences, Error> {
        let home_dir = env::home_dir().ok_or(HomeDirError)?;
        let mut file = OpenOptions::new()
            .read(true)
            .open(home_dir.join(".config/ded/config.toml"))?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        let preferences: Preferences = toml::from_str(&buf)?;
        Ok(preferences)
    }

    pub fn timestamp_fmt(&self) -> String {
        self.timestamp_fmt
            .clone()
            .unwrap_or_else(|| "%_I:%M".to_owned())
    }
}
