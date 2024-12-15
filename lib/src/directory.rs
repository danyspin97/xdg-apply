use std::{fs, path::PathBuf};

use eyre::{ensure, Result};
use serde::Deserialize;
use walkdir::WalkDir;

use crate::globals::HOME;
use crate::xdgvar::XdgVar;

#[derive(Deserialize)]
pub struct Directory {
    #[serde(rename = "old")]
    pub old_path: PathBuf,
    #[serde(rename = "new")]
    pub new_path: Option<PathBuf>,
    #[serde(default, rename = "type")]
    pub dir_type: Option<XdgVar>,
    #[serde(default, rename = "force-remove")]
    pub force_remove: bool,
    #[serde(default)]
    pub env: Option<String>,
}

impl Directory {
    pub fn rename(&self) -> Result<()> {
        let old_path = HOME.join(&self.old_path);
        let parent = self.dir_type.as_ref().unwrap_or(&XdgVar::Config);
        if old_path.exists() {
            ensure!(
                old_path.is_dir(),
                "{old_path:?} is not a directory but it's marked as such"
            );

            if let Some(new_path) = &self.new_path {
                WalkDir::new(&old_path)
                    .contents_first(true)
                    .into_iter()
                    .filter_map(|e| {
                        if let Err(err) = &e {
                            eprintln!("{err:?}");
                        }
                        e.ok()
                    })
                    .for_each(|entry| {
                        let entry_type = entry.file_type();
                        let entry_path = entry.path();
                        let new_path = parent
                            .get_dir()
                            .join(new_path)
                            .join(entry_path.strip_prefix(&old_path).unwrap());
                        if entry_type.is_file() {
                            fs::create_dir_all(new_path.parent().unwrap()).unwrap();
                            fs::rename(entry_path, new_path).unwrap();
                        } else if entry_type.is_dir() {
                            fs::remove_dir(entry_path).unwrap();
                        }
                    });
            } else {
                let empty = old_path.read_dir()?.count() == 0;
                if self.force_remove || empty {
                    fs::remove_dir_all(old_path)?;
                } else if !self.force_remove && !empty {
                    eprintln!("WARNING: directory {old_path:?} is legacy but is not empty")
                }
            }
        }
        Ok(())
    }

    pub fn get_env(self) -> Option<(String, PathBuf)> {
        if let Some(env) = self.env {
            if let Some(new_path) = self.new_path {
                let parent = self.dir_type.unwrap_or(XdgVar::Config);
                let new_path = parent.get_dir().join(new_path);
                if new_path.exists() {
                    Some((env, new_path))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}
