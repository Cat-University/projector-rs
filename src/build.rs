use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Deserialize, Serialize, Debug)]
pub struct Files {
    #[serde(rename = "rootScripts")]
    pub root_scripts: Vec<String>,
    pub modules: Vec<String>,
    pub css: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Build {
    #[serde(rename = "GLOBAL_ENV")]
    pub global_env: Map<String, Value>,
    pub date: String,
    pub number: u64,
    pub files: Files,
}
