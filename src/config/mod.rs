use std::{collections::BTreeMap, env, fs, path::PathBuf};

use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(transparent)]
pub struct DearConfig {
    pub profiles: BTreeMap<String, Profile>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct FileBackendConfig {
    pub path: PathBuf,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(tag = "type", content = "config")]
pub enum BackendConfig {
    #[serde(alias = "file")]
    File(FileBackendConfig),

    #[serde(alias = "memory")]
    InMemory,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Profile {
    pub backend: BackendConfig,
}

impl DearConfig {
    fn new(config_path: PathBuf) -> Self {
        let config_contents = fs::read_to_string(config_path).unwrap();
        println!("{config_contents:?}");
        let dear_config: Self = serde_yaml::from_str(&config_contents).unwrap();
        dear_config
    }
}

impl Default for DearConfig {
    fn default() -> Self {
        let home = env::var("HOME").unwrap();
        DearConfig::new(format!("{home}/.dear/.config").into())
    }
}

#[cfg(test)]
impl DearConfig {
    pub fn test_config() -> Self {
        let profile_name = "memory".to_owned();
        let profile = Profile {
            backend: BackendConfig::InMemory,
        };

        let mut map = BTreeMap::new();
        map.insert(profile_name, profile);
        Self { profiles: map }
    }
}
