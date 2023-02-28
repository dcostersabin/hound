use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub ecosystem: String,
}

impl Package {
    pub fn new(name: &str, version: &str, ecosystem: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            ecosystem: ecosystem.to_string(),
        }
    }
}
