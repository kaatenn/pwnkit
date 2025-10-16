use crate::error::PkError;
use std::path::{Path, PathBuf};

// Returns the data root directory.
// - Default: ~/.pwnkit
// - Test env: ./\.pwnkit (when PK_TEST_ROOT=1 or PK_ENV=test, or compiled with cfg(test))
pub fn root() -> PathBuf {
    let env_is_test = std::env::var("PK_ENV")
        .map(|v| v.eq_ignore_ascii_case("test"))
        .unwrap_or(false);
    let use_test_root = match std::env::var("PK_TEST_ROOT") {
        Ok(v) => v == "1" || v.eq_ignore_ascii_case("true"),
        Err(_) => cfg!(test) || env_is_test,
    };

    if use_test_root {
        PathBuf::from(".pwnkit")
    } else {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".pwnkit")
    }
}

// Lightweight config getter for now: reads from env only.
// Example: set WINDOWS username via `windows_username="YourName"` env var.
pub fn get_config(key: &str) -> Result<Option<String>, PkError> {
    if let Ok(val) = std::env::var(key) {
        return Ok(Some(val));
    }
    // Fallback to uppercased key commonly used for env vars
    if let Ok(val) = std::env::var(key.to_ascii_uppercase()) {
        return Ok(Some(val));
    }
    Ok(None)
}

// Resolve Windows Downloads path based on configured username.
// Checks both native Windows and WSL conventions.
pub fn windows_downloads_path() -> Result<PathBuf, PkError> {
    if let Some(username) = get_config("windows_username")? {
        let candidates = [
            Path::new("C:/Users").join(&username).join("Downloads"),
            Path::new("/mnt/c/Users").join(&username).join("Downloads"),
        ];
        for p in candidates {
            if p.exists() {
                return Ok(p);
            }
        }
        return Err(PkError::ConfigError(
            format!(
                "Downloads path for user '{}' not found. Please verify the username and environment.",
                username
            ),
        ));
    }
    Err(PkError::ConfigError(
        "Cannot determine Windows Downloads path. Please set env 'windows_username'.".to_string(),
    ))
}
