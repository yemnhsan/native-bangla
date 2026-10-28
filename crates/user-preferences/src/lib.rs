//! # user-preferences
//!
//! Configuration schemas and persistence for Native Bangla.

#![deny(unsafe_code)]

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PrefsError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("parse: {0}")]
    Parse(#[from] serde_json::Error),
    #[error("no config directory available")]
    NoConfigDir,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum InputLanguage {
    English,
    #[default]
    BanglaPhonetic,
    BanglaTraditional,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ThemeMode {
    Light,
    Dark,
    #[default]
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypingSettings {
    pub default_language: InputLanguage,
    pub layout_id: String,
    pub auto_space: bool,
    pub bengali_digits: bool,
    pub convert_danda: bool,
}

impl Default for TypingSettings {
    fn default() -> Self {
        Self {
            default_language: InputLanguage::BanglaPhonetic,
            layout_id: "phonetic".into(),
            auto_space: true,
            bengali_digits: false,
            convert_danda: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestionSettings {
    pub enabled: bool,
    pub max_suggestions: u8,
    pub next_word: bool,
    pub autocorrect: bool,
    pub autocorrect_aggressive: bool,
    pub animations: bool,
    pub number_key_select: bool,
}

impl Default for SuggestionSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            max_suggestions: 5,
            next_word: true,
            autocorrect: true,
            autocorrect_aggressive: false,
            animations: true,
            number_key_select: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceSettings {
    pub theme: ThemeMode,
    pub accent: String,
    pub candidate_opacity: f32,
    pub candidate_font_size: u8,
    pub compact_mode: bool,
    pub reduced_motion: bool,
}

impl Default for AppearanceSettings {
    fn default() -> Self {
        Self {
            theme: ThemeMode::System,
            accent: "#4F6BED".into(),
            candidate_opacity: 0.96,
            candidate_font_size: 14,
            compact_mode: false,
            reduced_motion: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalizationSettings {
    pub learning_enabled: bool,
}

impl Default for PersonalizationSettings {
    fn default() -> Self {
        Self {
            learning_enabled: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedSettings {
    pub start_with_windows: bool,
    pub minimize_to_tray: bool,
    pub performance_mode: bool,
    pub diagnostics: bool,
}

impl Default for AdvancedSettings {
    fn default() -> Self {
        Self {
            start_with_windows: true,
            minimize_to_tray: true,
            performance_mode: false,
            diagnostics: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutSettings {
    pub toggle_language: String,
    pub force_english_word: String,
}

impl Default for ShortcutSettings {
    fn default() -> Self {
        Self {
            toggle_language: "Ctrl+Space".into(),
            force_english_word: "Ctrl+E".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativeBanglaPreferences {
    pub version: u32,
    pub typing: TypingSettings,
    pub suggestions: SuggestionSettings,
    pub appearance: AppearanceSettings,
    pub personalization: PersonalizationSettings,
    pub advanced: AdvancedSettings,
    pub shortcuts: ShortcutSettings,
    /// Onboarding completed
    pub onboarding_complete: bool,
}

impl Default for NativeBanglaPreferences {
    fn default() -> Self {
        Self {
            version: 1,
            typing: TypingSettings::default(),
            suggestions: SuggestionSettings::default(),
            appearance: AppearanceSettings::default(),
            personalization: PersonalizationSettings::default(),
            advanced: AdvancedSettings::default(),
            shortcuts: ShortcutSettings::default(),
            onboarding_complete: false,
        }
    }
}

impl NativeBanglaPreferences {
    pub fn config_dir() -> Result<PathBuf, PrefsError> {
        ProjectDirs::from("org", "NativeBangla", "Native Bangla")
            .map(|d| d.config_dir().to_path_buf())
            .ok_or(PrefsError::NoConfigDir)
    }

    pub fn default_path() -> Result<PathBuf, PrefsError> {
        Ok(Self::config_dir()?.join("preferences.json"))
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self, PrefsError> {
        let data = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&data)?)
    }

    pub fn load_or_default(path: impl AsRef<Path>) -> Self {
        Self::load(path).unwrap_or_default()
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), PrefsError> {
        if let Some(parent) = path.as_ref().parent() {
            fs::create_dir_all(parent)?;
        }
        let data = serde_json::to_string_pretty(self)?;
        fs::write(path, data)?;
        Ok(())
    }

    pub fn load_user() -> Result<Self, PrefsError> {
        let path = Self::default_path()?;
        if path.exists() {
            Self::load(&path)
        } else {
            let prefs = Self::default();
            prefs.save(&path)?;
            Ok(prefs)
        }
    }
}

pub const PRODUCT_NAME: &str = "Native Bangla";
pub const TAGLINE: &str = "Type Naturally. Write Beautifully.";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_json() {
        let p = NativeBanglaPreferences::default();
        let s = serde_json::to_string(&p).unwrap();
        let q: NativeBanglaPreferences = serde_json::from_str(&s).unwrap();
        assert_eq!(q.version, 1);
        assert_eq!(PRODUCT_NAME, "Native Bangla");
    }
}
