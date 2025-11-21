#![cfg(not(debug_assertions))]

use codex_core::config::Config;

/// Update checks are disabled in this build, so surface nothing.
pub fn get_upgrade_version(config: &Config) -> Option<String> {
    let _ = config;
    None
}

/// Popup banner is also disabled.
pub fn get_upgrade_version_for_popup(config: &Config) -> Option<String> {
    let _ = config;
    None
}

/// No-op dismissal writing because there is nothing to persist.
pub async fn dismiss_version(config: &Config, version: &str) -> anyhow::Result<()> {
    let _ = (config, version);
    Ok(())
}
