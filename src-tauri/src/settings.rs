use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

const SETTINGS_FILE: &str = "settings.toml";
const SETTINGS_VERSION: u32 = 1;
const SETTINGS_DIR: &str = "not4granted";

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Light,
    Dark,
    System,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct Settings {
    pub version: u32,
    pub theme: Theme,
    pub locale: String,
    pub currency: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            version: SETTINGS_VERSION,
            theme: Theme::System,
            locale: "en-US".to_string(),
            currency: "EUR".to_string(),
        }
    }
}

fn settings_path(app: &AppHandle) -> Result<PathBuf, String> {
    let config_dir = app
        .path()
        .app_config_dir()
        .map_err(|error| format!("Could not locate application config directory: {error}"))?;

    Ok(config_dir.join(SETTINGS_DIR).join(SETTINGS_FILE))
}

const SUPPORTED_LOCALES: &[&str] = &[
    "en-US",
    "fi-FI",
    "sv-SE",
    "es-ES",
    "fr-FR",
    "de-DE",
    "it-IT",
    "pt-PT",
    "nl-NL",
    "pl-PL",
    "ru-RU",
    "ja-JP",
    "zh-CN",
    "ko-KR",
    "ar-SA",
    "he-IL",
    "tr-TR",
    "cs-CZ",
    "da-DK",
    "el-GR",
    "hu-HU",
    "id-ID",
    "ms-MY",
    "no-NO",
    "ro-RO",
    "sk-SK",
    "th-TH",
    "uk-UA",
    "vi-VN",
];

fn validate_settings(settings: &Settings) -> Result<(), String> {
    if settings.version != SETTINGS_VERSION {
        return Err(format!(
            "Unsupported settings version: {}. Expected: {}",
            settings.version, SETTINGS_VERSION
        ));
    }

    if !SUPPORTED_LOCALES.contains(&settings.locale.as_str()) {
        return Err(format!(
            "Unsupported locale: {}. Supported locales are: {:?}",
            settings.locale, SUPPORTED_LOCALES
        ));
    }

    if settings.currency.trim().is_empty() {
        return Err("Currency cannot be empty".to_string());
    } else if settings.currency.len() != 3 || !settings.currency.chars().all(|c| c.is_ascii_alphabetic()) {
        return Err(format!(
            "Invalid currency code: {}. It must be a 3-letter ISO 4217 code.",
            settings.currency
        ));
    }

    Ok(())
}

pub fn load_settings(app: &AppHandle) -> Result<Settings, String> {
    let settings_path = settings_path(app)?;
    if !settings_path.exists() {
        let default_settings = Settings::default();
        save_settings(app, default_settings.clone())?;
        return Ok(default_settings);
    }

    let settings_content = fs::read_to_string(&settings_path)
        .map_err(|e| format!("Failed to read settings file: {}", e))?;
    let settings: Settings = toml::from_str(&settings_content)
        .map_err(|e| format!("Failed to parse settings file: {}", e))?;

    validate_settings(&settings)?;

    Ok(settings)
}

pub fn save_settings(app: &AppHandle, mut settings: Settings) -> Result<Settings, String> {
    settings.version = SETTINGS_VERSION; // Ensure the version is always set to the current version

    validate_settings(&settings)?;

    let settings_path = settings_path(app)?;
    let settings_content = toml::to_string_pretty(&settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))?;
    if let Some(parent) = settings_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create settings directory: {}", e))?;
    }
    fs::write(&settings_path, settings_content)
        .map_err(|e| format!("Failed to write settings file: {}", e))?;

    Ok(settings)
}