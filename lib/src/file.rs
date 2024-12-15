use std::{fs, path::PathBuf};

use eyre::{ensure, ContextCompat, Result};
use serde::Deserialize;

use crate::globals::HOME;
use crate::xdgvar::XdgVar;

#[derive(Deserialize)]
pub struct File {
    #[serde(rename = "old")]
    pub old_path: PathBuf,
    #[serde(rename = "new")]
    pub new_path: PathBuf,
    #[serde(default, rename = "type")]
    pub file_type: Option<XdgVar>,
    #[serde(default)]
    pub env: Option<String>,
}

impl File {
    pub fn rename(&self) -> Result<()> {
        let old_path = HOME.join(&self.old_path);
        let parent = self.file_type.clone().unwrap_or(XdgVar::Config);
        let new_path = parent.get_dir().join(&self.new_path);

        if old_path.exists() {
            ensure!(
                old_path.is_file(),
                "{old_path:?} is a directory but it's marked as file",
            );
            ensure!(!new_path.exists(), "{new_path:?} already exists");
            let new_parent = new_path
                .parent()
                .with_context(|| format!("unable to get parent directory of {new_path:?}"))?;
            if !new_parent.exists() {
                fs::create_dir_all(new_parent)?;
            }
            fs::rename(old_path, &new_path)?;
        }

        Ok(())
    }

    pub fn get_env(self) -> Option<(String, PathBuf)> {
        let parent = self.file_type.unwrap_or(XdgVar::Config);
        let new_path = parent.get_dir().join(self.new_path);

        if let Some(env) = self.env {
            if new_path.exists() {
                Some((env, new_path))
            } else {
                None
            }
        } else {
            None
        }
    }
}
