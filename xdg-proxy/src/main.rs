use std::{collections::HashMap, os::unix::process::CommandExt, path::Path};

use lib::application::Application;
use xdg::BaseDirectories;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let binary = args[0].clone();
    let binary = Path::new(&binary).file_name().unwrap().to_str().unwrap();

    let xdg = BaseDirectories::new().unwrap();

    let apps: Vec<(String, String)> = serde_yaml::from_str::<HashMap<String, Application>>(
        include_str!("../../applications.yml"),
    )
    .unwrap()
    .into_values()
    .flat_map(|app| {
        app.aliases
            .into_iter()
            .map(|alias| (alias.exe, alias.arguments))
            .collect::<Vec<_>>()
    })
    .collect();

    let path = std::env::var("PATH").unwrap_or_default();
    let xdg_apply_aliases_dir = xdg
        .create_state_directory("xdg-apply/aliases")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    // Remove the directory from the PATH, otherwise we would have a recursive call
    if path.contains(&xdg_apply_aliases_dir) {
        let path = path.replace(&xdg_apply_aliases_dir, "");
        std::env::set_var("PATH", path);
    }
    if let Some((_, xdg_run_args)) = apps
        .iter()
        .find(|(exe, _)| *exe == binary)
    {
        let mut cmd = std::process::Command::new(binary);
        let injected_args = xdg_run_args
            .split_whitespace()
            .map(|arg| {
                let mut arg =
                    arg.replace("$XDG_CONFIG_HOME", xdg.get_config_home().to_str().unwrap());
                arg = arg.replace("$XDG_DATA_HOME", xdg.get_data_home().to_str().unwrap());
                arg = arg.replace("$XDG_CACHE_HOME", xdg.get_cache_home().to_str().unwrap());
                arg = arg.replace("$XDG_STATE_HOME", xdg.get_state_home().to_str().unwrap());
                arg
            })
            .collect::<Vec<_>>();
        cmd.args(injected_args);
        cmd.args(&args[1..]);
        let err = cmd.exec();
        eprintln!("error: {:?}", err);
    }
}
