use std::path::PathBuf;

use crate::globals::XDG_DIRS;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum XdgVar {
    Config,
    Data,
    Cache,
    State,
}

impl XdgVar {
    pub fn get_dir(&self) -> PathBuf {
        match self {
            XdgVar::Config => XDG_DIRS.get_config_home(),
            XdgVar::Data => XDG_DIRS.get_data_home(),
            XdgVar::Cache => XDG_DIRS.get_cache_home(),
            XdgVar::State => XDG_DIRS.get_state_home(),
        }
    }
}
