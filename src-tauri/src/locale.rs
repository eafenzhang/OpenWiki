//! Unified locale resolver for the app.
//! Reads the user's language_mode from the database and resolves it to a concrete locale.
//! Used by AI prompts, background tasks, and any backend code that needs the current locale.

use crate::storage::database::Database;
use crate::storage::repository::Repository;
use std::sync::Arc;

/// Supported locales.
pub const LOCALE_ZH_CN: &str = "zh-CN";
pub const LOCALE_EN_US: &str = "en-US";

/// Resolve the current app locale from persisted settings.
/// Falls back to zh-CN if no setting is found.
pub fn resolve_locale(db: &Arc<Database>) -> String {
    let repo = Repository::new(db.clone());
    match repo.get_setting("language_mode") {
        Ok(Some(mode)) => resolve_from_mode(&mode),
        _ => LOCALE_ZH_CN.to_string(),
    }
}

/// Resolve a language mode string to a concrete locale.
fn resolve_from_mode(mode: &str) -> String {
    match mode {
        "en-US" => LOCALE_EN_US.to_string(),
        "zh-CN" => LOCALE_ZH_CN.to_string(),
        "system" => system_locale(),
        _ => LOCALE_ZH_CN.to_string(),
    }
}

/// Detect system locale. On macOS, reads the AppleLocale / AppleLanguages defaults.
/// Falls back to zh-CN.
fn system_locale() -> String {
    // Try to detect from environment
    if let Ok(lang) = std::env::var("LANG") {
        if lang.starts_with("en") {
            return LOCALE_EN_US.to_string();
        }
    }
    // On macOS, try defaults read
    if let Ok(output) = std::process::Command::new("defaults")
        .args(["read", "-g", "AppleLocale"])
        .output()
    {
        let locale = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if locale.starts_with("en") {
            return LOCALE_EN_US.to_string();
        }
    }
    LOCALE_ZH_CN.to_string()
}

/// Check if the current locale is English.
pub fn is_english(locale: &str) -> bool {
    locale.starts_with("en")
}
