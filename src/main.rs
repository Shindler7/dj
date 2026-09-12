//! Console utility for running Django applications.
//!
//! This tool expects a `start.toml` configuration file to be present.
//! Here's a sample configuration with all available options:
//!
//! ```toml
//! [environment]
//! env = ".env"
//!
//! [django]
//! runserver = ["python", "-m", "uvicorn", "--reload", "rustogrped.asgi:application"]
//! port = 8080
//! ipv6 = false
//! nothreading = false
//! noreload = false
//! nostatic = false
//! insecure = false
//! skip_checks = false
//!
//! [tuna]
//! project = "brainstorm"
//! config = "dev"
//! api_key = "${TUNA_API_KEY}"
//! ```
//!
//! All sections are optional — if you omit any (or all) of them,
//! the tool falls back to sensible defaults. The example above
//! shows the default values for every field.
mod cli;
mod commands;
mod constants;
mod executor;
mod parse_toml;

use anyhow::{Context, Result as AnyhowResult};
use cli::{Command as ArgsCommand, parse_args};
use env_logger::WriteStyle;
use log::{LevelFilter, error};
use parse_toml::read_params;
use std::{env, fs, io::Write, process::ExitCode};

/// Application entry point.
///
/// Initializes logging, runs the main application logic, and returns
/// an appropriate exit code.
fn main() -> ExitCode {
    let app_name = env!("CARGO_PKG_NAME").to_uppercase();

    env_logger::builder()
        .format_timestamp(None)
        .format(move |buf, record| {
            writeln!(buf, "[{} | {}] {}", app_name, record.level(), record.args())
        })
        .write_style(WriteStyle::Auto)
        .filter_level(LevelFilter::Info)
        .init();

    dj_start().unwrap_or_else(|err| {
        error!("{err:#}");
        ExitCode::FAILURE
    })
}

/// Core application logic.
///
/// Parses command-line arguments and dispatches to the requested command:
/// `init`, `runserver`, `manage`, or `example`.
fn dj_start() -> AnyhowResult<ExitCode> {
    let command = parse_args().command;

    if let ArgsCommand::Init { force } = command {
        return init_dj(force);
    }

    let params = read_params()?;
    log::debug!("Configuration loaded successfully.");

    match command {
        ArgsCommand::Runserver => executor::run_server(&params),
        ArgsCommand::Manage(django_args) => {
            let django_commands = django_args.into();
            executor::manage(&params, &django_commands)
        }
        ArgsCommand::Example { path, args } => executor::example(&params, &path, &args),
        ArgsCommand::Init { .. } => unreachable!(),
    }
}

/// Creates a default `start.toml` configuration file in the current directory.
///
/// If the file already exists and `force` is `false`, the function logs an
/// error and returns `ExitCode::FAILURE` without modifying anything.
fn init_dj(force: bool) -> AnyhowResult<ExitCode> {
    let start_toml = constants::toml_path()?;
    if start_toml.is_file() && !force {
        log::error!(
            "`{}` already exists. Use `--force` to overwrite.",
            start_toml.display()
        );
        return Ok(ExitCode::FAILURE);
    }

    fs::write(&start_toml, constants::DEFAULT_TOML_CONTENT)
        .with_context(|| format!("Failed to write `{}`", start_toml.display()))?;

    log::info!("Created `{}`", start_toml.display());
    log::info!("Edit the file, then run `dj runserver` (or just `dj`).");

    Ok(ExitCode::SUCCESS)
}
