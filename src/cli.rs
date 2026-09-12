//! Command-line argument parsing.
//!
//! Defines the CLI structure for launching Brainstorm in development mode
//! with optional Tuna cloud secrets integration.

use clap::{Parser, Subcommand, error::ErrorKind};
use std::path::PathBuf;

/// Launch App in development mode with Tuna cloud secrets.
#[derive(Debug)]
pub(crate) struct DjUp {
    pub(crate) command: Command,
}

/// Internal CLI structure used for parsing command-line arguments.
#[derive(Debug, Parser)]
#[command(
    version,
    about,
    long_about = None,
    override_usage = "dj [OPTIONS] [COMMAND] [ARGS]...",
    after_help = format!(r#"{bold}{underline}Django Manage Proxy Modes{reset}:

Any unknown command is automatically proxied to `manage.py`.

Examples:
  dj <command>    Proxy any arbitrary command to python manage.py <command>
  dj m            Alias for `migrate`
  dj mm           Alias for `makemigrations`
  dj s            Alias for `shell`

{bold}{underline}Example Modes{reset}:

Run custom Python scripts with the same environment and features.

Usage:
  dj example <SCRIPT> [ARGS]...

Examples:
  dj example sandbox/check_llm.py
  dj example ./my_script.py --name=world

All features (tuna, uv) are available in Example mode just like in `runserver`."#,
    bold="\x1b[1m",
    underline="\x1b[4m",
    reset="\x1b[0m")
)]
struct Cli {
    /// subcommand to execute (defaults to `runserver`)
    #[command(subcommand)]
    command: Option<Command>,
}

/// Parse CLI args into a `DjUp` struct.
///
/// - `init` — creates a default `start.toml` (use `--force` to overwrite).
/// - `runserver` / `run` — starts the Django dev server (default).
/// - `example <path>` — runs a Python script with the same environment.
/// - Anything else is proxied to `manage.py` with the raw args.
#[derive(Debug, Subcommand, Default)]
pub(crate) enum Command {
    /// Create a default `start.toml` in the current directory.
    Init {
        /// Overwrite an existing `start.toml`.
        #[arg(long)]
        force: bool,
    },

    /// Start the Django development server (default).
    #[default]
    #[command(visible_alias = "run")]
    Runserver,

    /// Run any `manage.py` command — just pass it along.
    #[command(external_subcommand)]
    Manage(Vec<String>),

    /// Run an example Python script with the same environment and features.
    Example {
        /// Path to the Python script to execute.
        path: PathBuf,

        /// Arguments to pass to the script (optional).
        #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
        args: Vec<String>,
    },
}

/// Parse command-line arguments and return the configuration.
///
/// If no subcommand is provided, `runserver` is used as the default.
pub(crate) fn parse_args() -> DjUp {
    let mut command = match Cli::try_parse() {
        Ok(cli) => cli.command.unwrap_or_default(),
        Err(err) => {
            if err.kind() == ErrorKind::DisplayHelp
                || err.kind() == ErrorKind::DisplayVersion
                || err.kind() == ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
            {
                err.exit()
            }

            // Fallback: anything else becomes `manage` with the raw args.
            let raw_args: Vec<String> = std::env::args().skip(1).collect();
            Command::Manage(raw_args)
        }
    };

    if let Command::Manage(ref mut args) = command
        && let Some(first_arg) = args.first()
    {
        let expanded = match first_arg.as_str() {
            "m" => Some("migrate"),
            "mm" => Some("makemigrations"),
            "s" => Some("shell"),
            _ => None,
        };

        if let Some(replacement) = expanded {
            args[0] = replacement.to_string();
        }
    }

    DjUp { command }
}
