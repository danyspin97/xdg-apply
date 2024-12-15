use serde::Deserialize;

#[derive(Deserialize)]
pub struct Alias {
    pub exe: String,
    #[serde(rename = "args")]
    pub arguments: String,
}
