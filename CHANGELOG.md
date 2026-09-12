# Changelog

All notable changes to this project will be documented in this file.

The format is based
on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.6] — 2026-09-12

**Added**: `dj init` command with `--force` flag to create `start.toml`

**Changed**: config loading refactored; `runserver` and `example` commands arg
handling improved

## [0.1.5] — 2026-09-09

### Added

- **New constant** `DEFAULT_RUN_DJANGO` – Centralized default runserver command
  string

### Fixed

- **Critical: incorrect order of default Django server command arguments** —
  the argument sequence was malformed, preventing the application from
  starting. Command construction now produces the correct order for
  `runserver`.

### Changed

- **Refactored `DjangoCommands` struct**:
    - Migrated from tuple struct `Vec<String>` to a named field
      `command: Vec<String>`
    - Improved readability and maintainability with explicit field access
- **Renamed method** – `is_default()` → `is_default_run()` for better clarity
- **Improved error handling in `command_execute`**:
    - Replaced `eprintln!` with structured `log::error!` and `log::info!`
      macros
    - Added `anyhow::Context` for better error context
    - Returns `AnyhowResult<ExitCode>` instead of raw `ExitCode`
- **Logging improvements**:
    - Inlined format arguments in log macros (`writeln!` without positional
      args)
    - Added logging for command start (`"Running command..."`) and completion
      (`"Command completed successfully."`)
    - Changed error output to use `{err:#}` for more detailed error formatting

## [0.1.4] — 2026-09-07

### Added

- Configured a custom Clippy lint suite under `[lints.clippy]` with `pedantic`
  group at lower priority (`priority = -1`).
- Enabled strict safety lints: `clone_on_ref_ptr`, `unwrap_used`, and
  `expect_used` as warnings.
- Explicitly allowed noisy pedantic lints: `must_use_candidate`,
  `missing_errors_doc`,
  `missing_panics_doc`, and `module_name_repetitions`.
- Added localized `#[allow(clippy::struct_excessive_bools)]` for the `Django`
  configuration struct.

### Changed

- Refactored `Django` configuration struct to support field-level Serde
  defaults alongside a manual `impl Default` implementation to resolve
  dependency trait bounds.

### Fixed

- Migrated Clippy configurations from `[lints.rust]` to `[lints.clippy]` to fix
  Rust 1.98+ deprecation warnings.
- Resolved `uninlined_format_args` warnings by modernizing format strings
  across logging macros.

### Fixed

- Fixed a compiler warning introduced in Rust 1.98+ regarding the deprecated
  syntax of declaring Clippy lints inside the `[lints.rust]` section. All lints
  have been properly separated into their dedicated `[lints.clippy]` block.
- Resolved `uninlined_format_args` warnings across the logging infrastructure
  by modernizing format strings (e.g., updating from `"{:?}", command` to
  `"{command:?}"`).

## [0.1.3] — 2026-09-06

### Added

- `example` command for running custom Python scripts (`dj example <SCRIPT>`)
- Better help output with usage examples and proxy mode description

### Changed

- Code cleanup (Clippy fixes, removed redundant code)
