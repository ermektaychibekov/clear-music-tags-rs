// src/config.rs
use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    #[serde(default = "default_remove_strings")]
    pub remove_strings: Vec<String>,
    #[serde(default = "default_paths")]
    pub paths: Vec<String>,
}

fn default_remove_strings() -> Vec<String> {
    vec![
        "https://djsoundtop.com".into(),
        "https://electronicfresh.com".into(),
        "djsoundtop.com".into(),
        "electronicfresh.com".into(),
    ]
}

fn default_paths() -> Vec<String> {
    vec![".".into()]
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let content = fs::read_to_string(path)?;
        Ok(serde_yaml::from_str(&content)?)
    }

    pub fn default() -> Self {
        Self {
            remove_strings: default_remove_strings(),
            paths: default_paths(),
        }
    }
}
