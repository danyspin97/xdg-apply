use std::path::PathBuf;
use std::sync::atomic::AtomicBool;

lazy_static! {
    pub static ref XDG_DIRS: xdg::BaseDirectories = xdg::BaseDirectories::new().unwrap();
    pub static ref HOME: PathBuf = dirs::home_dir().unwrap();
    pub static ref SUCCESS: AtomicBool = AtomicBool::new(true);
}
