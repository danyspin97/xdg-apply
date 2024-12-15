use std::{collections::HashMap, fs, process, sync::atomic::Ordering};

use color_eyre::eyre::Context;
use lib::{
    application::Application,
    globals::{SUCCESS, XDG_DIRS},
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use xdg::BaseDirectories;

fn main() {
    let xdg_apply_dir = BaseDirectories::with_prefix("xdg-apply").unwrap();

    let apps = serde_yaml::from_str::<HashMap<String, Application>>(include_str!(
        "../../applications.yml"
    ))
    .unwrap()
    .into_iter()
    .map(|(name, mut app)| {
        app.apply_presets(&name);
        app
    })
    .collect::<Vec<Application>>();
    apps.par_iter().for_each(Application::handle_files);
    let aliases = apps
        .par_iter()
        .map(|app| {
            app.aliases
                .iter()
                .map(|alias| &alias.exe)
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();
    let alias_dir = xdg_apply_dir.create_state_directory("aliases").unwrap();
    // Remove the directory for lingering aliases
    fs::remove_dir_all(&alias_dir).unwrap();
    // and recreate it
    xdg_apply_dir.create_state_directory("aliases").unwrap();
    for alias in aliases {
        std::os::unix::fs::symlink(
            "/home/danyspin97/proj/coding/xdg-apply/target/debug/xdg-proxy",
            alias_dir.join(alias),
        )
        .with_context(|| format!("unable to create symlink {:?}", alias_dir.join(alias)))
        .unwrap();
    }

    let env = apps.into_iter().flat_map(Application::get_env);

    let mut environment = "export XDG_CONFIG_HOME=${HOME}/.config
export XDG_DATA_HOME=${HOME}/.local/share
export XDG_CACHE_HOME=${HOME}/.cache"
        .to_string();

    let exported_env = env
        .into_iter()
        .map(|(var, val)| format!("export {var}={val:?}"))
        .collect::<Vec<String>>()
        .join("\n");
    environment.push('\n');
    environment.push_str(&exported_env);

    fs::write(
        XDG_DIRS.place_data_file("environment").unwrap(),
        &environment,
    )
    .unwrap();

    process::exit(if SUCCESS.load(Ordering::Acquire) {
        0
    } else {
        1
    });
}
