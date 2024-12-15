use serde::Deserialize;

#[derive(Deserialize)]
pub enum Preset {
    #[serde(rename = "config-dir")]
    ConfigDir,
    #[serde(rename = "legacy-dir")]
    LegacyConfigDir,
    #[serde(rename = "rc-file")]
    RcFile,
    #[serde(rename = "config-file")]
    ConfigFile,
}
