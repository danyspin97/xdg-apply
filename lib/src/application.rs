use std::path::PathBuf;
use std::sync::atomic::Ordering;

use serde::Deserialize;

use crate::globals::SUCCESS;
use crate::xdgvar::XdgVar;
use crate::{alias::Alias, directory::Directory, file::File, preset::Preset};

#[derive(Deserialize)]
pub struct Application {
    #[serde(default)]
    pub aliases: Vec<Alias>,
    #[serde(default)]
    files: Vec<File>,
    #[serde(default)]
    dirs: Vec<Directory>,
    #[serde(default)]
    presets: Vec<Preset>,
}

impl Application {
    pub fn handle_files(&self) {
        // Move files
        self.files.iter().map(File::rename).for_each(|res| {
            if let Err(e) = res {
                eprintln!("E: {e:?}");
                SUCCESS.store(false, Ordering::Relaxed);
            }
        });
        // Move/remove empty dirs
        self.dirs.iter().map(Directory::rename).for_each(|res| {
            if let Err(e) = res {
                eprintln!("E: {e:?}");
                SUCCESS.store(false, Ordering::Relaxed);
            }
        });
    }

    pub fn get_env(self) -> Vec<(String, PathBuf)> {
        self.files
            .into_iter()
            .filter_map(File::get_env)
            .chain(self.dirs.into_iter().filter_map(Directory::get_env))
            .collect()
    }

    pub fn apply_presets(&mut self, name: &str) {
        for preset in &self.presets {
            match preset {
                Preset::ConfigDir => self.dirs.push(Directory {
                    old_path: PathBuf::from(format!(".{name}")),
                    new_path: Some(PathBuf::from(name.to_string())),
                    dir_type: None,
                    force_remove: false,
                    env: None,
                }),
                Preset::RcFile => self.files.push(File {
                    old_path: PathBuf::from(format!(".{name}rc")),
                    new_path: PathBuf::from(format!("{name}/{name}rc")),
                    file_type: None,
                    env: None,
                }),
                Preset::LegacyConfigDir => self.dirs.push(Directory {
                    old_path: PathBuf::from(format!(".{name}")),
                    new_path: None,
                    dir_type: None,
                    force_remove: false,
                    env: None,
                }),
                Preset::ConfigFile => self.files.push(File {
                    old_path: PathBuf::from(format!(".{name}")),
                    new_path: PathBuf::from(format!("{name}/{name}.conf")),
                    file_type: None,
                    env: None,
                }),
            }
        }
    }

    pub fn has_data(&self) -> bool {
        self.files.iter().any(|file| {
            file.file_type
                .clone()
                .unwrap_or(XdgVar::Config)
                .get_dir()
                .join(&file.new_path)
                .exists()
        }) || self
            .dirs
            .iter()
            .filter(|dir| dir.new_path.is_some())
            .any(|dir| {
                dir.dir_type
                    .clone()
                    .unwrap_or(XdgVar::Config)
                    .get_dir()
                    .join(dir.new_path.as_ref().unwrap())
                    .exists()
            })
    }
}
