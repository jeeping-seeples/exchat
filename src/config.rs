//! Persistent settings: providers, their models, and which model fills which role.

use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

/// An OpenAI-compatible API endpoint with the models you want available from it.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Provider {
    pub name: String,
    pub base_url: String,
    pub api_key: String,
    pub models: Vec<String>,
}

/// A specific model on a specific provider.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModelRef {
    pub provider: String,
    pub model: String,
}

/// Which model does which job.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Roles {
    /// Rewrites the prompt and proposes search queries.
    pub refiner: Option<ModelRef>,
    /// Decides whether gathered evidence is enough.
    pub judge: Option<ModelRef>,
    /// Writes the final answer and handles follow-up chat.
    pub explainer: Option<ModelRef>,
    /// Fast model that names new chats.
    pub titler: Option<ModelRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Settings {
    pub providers: Vec<Provider>,
    pub roles: Roles,
    pub max_search_rounds: u8,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            providers: Vec::new(),
            roles: Roles::default(),
            max_search_rounds: 4,
        }
    }
}

impl Settings {
    pub fn path() -> Result<PathBuf> {
        let dirs =
            ProjectDirs::from("", "", "exchat").context("could not find a home directory")?;
        Ok(dirs.config_dir().join("settings.json"))
    }

    /// Load settings, or return defaults if the file does not exist yet.
    pub fn load() -> Result<Self> {
        let path = Self::path()?;
        if !path.exists() {
            return Ok(Self::default());
        }
        let text =
            fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
        serde_json::from_str(&text).with_context(|| format!("parsing {}", path.display()))
    }

    /// Write atomically (temp file + rename). Contains API keys, so lock it down on unix.
    pub fn save(&self) -> Result<()> {
        let path = Self::path()?;
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }
        let tmp = path.with_extension("json.tmp");
        fs::write(&tmp, serde_json::to_string_pretty(self)?)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&tmp, fs::Permissions::from_mode(0o600))?;
        }
        fs::rename(&tmp, &path)?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn provider(&self, name: &str) -> Option<&Provider> {
        self.providers.iter().find(|p| p.name == name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_through_json() {
        let mut s = Settings::default();
        s.providers.push(Provider {
            name: "example".into(),
            base_url: "https://api.example.com/v1".into(),
            api_key: "key".into(),
            models: vec!["a".into(), "b".into()],
        });
        s.roles.explainer = Some(ModelRef {
            provider: "example".into(),
            model: "b".into(),
        });
        let json = serde_json::to_string(&s).unwrap();
        let back: Settings = serde_json::from_str(&json).unwrap();
        assert_eq!(s, back);
    }

    #[test]
    fn missing_fields_fall_back_to_defaults() {
        let s: Settings = serde_json::from_str("{}").unwrap();
        assert_eq!(s.max_search_rounds, 4);
        assert!(s.providers.is_empty());
    }
}
