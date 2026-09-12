//! Command execution logic — runs Django server and management commands.
//!
//! This module handles building the final command line, prepending feature
//! wrappers (Tuna, uv), and executing the resulting process.

use crate::{
    commands::{tuna_args, uv_args},
    constants::{DEFAULT_RUN_DJANGO, MANAGE_PY, PYTHON_BIN},
    parse_toml::{DjangoCommands, Params},
};
use anyhow::{Result as AnyhowResult, bail};
use std::{
    path::Path,
    process::{Command, ExitCode, ExitStatus, Stdio},
};

/// Starts the Django development server with the provided configuration.
pub(super) fn run_server(params: &Params) -> AnyhowResult<ExitCode> {
    let mut django_commands = params.django.runserver();

    // Default command if no custom command is provided.
    if django_commands.is_empty() {
        let mut default_args = vec![
            MANAGE_PY.to_string(),
            DEFAULT_RUN_DJANGO.to_string(),
            params.django.port.to_string(),
        ];

        default_args.extend(params.django.runserver_args());

        django_commands = default_args.into();
    } else {
        log::info!(
            "custom command detected — [django] section settings (port, flags, etc.) are ignored."
        );
    }

    wrap_and_execute(django_commands, params)
}

/// Executes a `manage.py` command with the given arguments.
pub(super) fn manage(params: &Params, django_args: &DjangoCommands) -> AnyhowResult<ExitCode> {
    let mut django_commands = DjangoCommands::new();

    if !params.features.uv {
        django_commands.push(PYTHON_BIN.to_string());
    }

    django_commands.push(MANAGE_PY.to_string());
    django_commands.extend(django_args.iter().cloned());

    wrap_and_execute(django_commands, params)
}

/// Executes a custom Python script with the same environment and feature wrappers.
///
/// ## Arguments
// - `params` — Application configuration (features, Tuna params, etc.)
// - `script` — Path to the Python script to execute
// - `args` — Additional arguments to pass to the script
pub(super) fn example(params: &Params, script: &Path, args: &[String]) -> AnyhowResult<ExitCode> {
    if !script.is_file() {
        bail!("script file not found: {}", script.display());
    }

    let mut django_commands = DjangoCommands::new();

    if !params.features.uv {
        django_commands.push(PYTHON_BIN.to_string());
    }

    django_commands.push(script.to_string_lossy().to_string());
    django_commands.extend(args.iter().cloned());

    wrap_and_execute(django_commands, params)
}

/// Prepends feature-related commands (Tuna, uv) to the Django command list.
///
/// If Tuna is enabled, it injects `tuna secrets run ...` with credentials.
/// If uv is enabled, it injects `uv run ...`.
///
/// Feature commands are always prepended before the actual Django command.
fn update_commands_by_features(
    django_commands: DjangoCommands,
    params: &Params,
) -> AnyhowResult<DjangoCommands> {
    let mut features = DjangoCommands::new();

    // Tuna.
    if params.features.tuna {
        match &params.tuna {
            Some(tuna_params) => features.extend(tuna_args(tuna_params)),
            None => bail!(
                "Tuna feature is enabled (`features.tuna = true`), \
                but the `[tuna]` configuration block is missing. \
                Either add a `[tuna]` section to your `start.toml` or set `features.tuna = false`."
            ),
        }
    }

    // Uv.
    if params.features.uv {
        features.extend(uv_args());
    }

    // Return without features.
    if features.is_empty() {
        return Ok(django_commands);
    }

    features.extend(django_commands);

    Ok(features)
}

/// Wraps the command with feature flags (Tuna, uv) and executes it.
///
/// This is the common execution path for all commands — it applies
/// feature wrappers and spawns the final process.
fn wrap_and_execute(django_commands: DjangoCommands, params: &Params) -> AnyhowResult<ExitCode> {
    let django_commands = update_commands_by_features(django_commands, params)?;
    command_execute(django_commands)
}

/// Spawns a child process, waits for it to complete, and returns the exit status.
fn command_execute(django_command: DjangoCommands) -> AnyhowResult<ExitCode> {
    let mut command = Command::try_from(django_command)?;
    log::info!("Running command...");

    command
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    match command.spawn() {
        Ok(mut child) => match child.wait() {
            Ok(status) if status.success() => {
                log::info!("Command completed successfully.");
                Ok(ExitCode::SUCCESS)
            }
            Ok(status) => {
                log::error!("Command failed: {}", format_exit_status(status));
                Ok(ExitCode::FAILURE)
            }
            Err(err) => {
                log::error!(
                    "Failed to run `{}`: {err}",
                    command.get_program().to_string_lossy()
                );
                Ok(ExitCode::FAILURE)
            }
        },
        Err(err) => {
            eprintln!("Failed to run command `{command:?}`: {err}");
            Ok(ExitCode::FAILURE)
        }
    }
}

/// Formats a process exit status into a human-readable string.
#[must_use]
fn format_exit_status(status: ExitStatus) -> String {
    status.code().map_or_else(
        || "terminated".to_string(),
        |code| format!("exit code `{code}`"),
    )
}
