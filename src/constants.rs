//! Application constants and configuration file paths.

use anyhow::{Context, Result as AnyhowResult};
use std::{env, path::PathBuf};

// Name of the configuration file containing startup parameters.
pub(crate) const TOML_NAME: &str = "start.toml";

/// Returns the full path to the configuration file (`start.toml`).
///
/// Resolves the path relative to the current working directory.
pub(crate) fn toml_path() -> AnyhowResult<PathBuf> {
    let toml = PathBuf::new().join(current_dir()?).join(TOML_NAME);

    Ok(toml)
}

/// Returns the current working directory.
fn current_dir() -> AnyhowResult<PathBuf> {
    env::current_dir().context("Could not get current directory")
}

pub(crate) const DEFAULT_TOML_CONTENT: &str = r"[django]
port = 8000
";

/// The default Django management script name.
pub(crate) const MANAGE_PY: &str = "manage.py";

/// Python executable name — platform-specific.
/// On Windows, use `py` (Python launcher). On Unix-like systems, use `python`.
#[cfg(windows)]
pub(crate) const PYTHON_BIN: &str = "py";

#[cfg(not(windows))]
pub(crate) const PYTHON_BIN: &str = "python3";

/// Default Django management command for starting the development server.
pub(crate) const DEFAULT_RUN_DJANGO: &str = "runserver";
