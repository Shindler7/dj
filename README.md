![GitHub Release](https://img.shields.io/github/v/release/Shindler7/dj)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/shindler7/dj)
![GitHub commits since latest release](https://img.shields.io/github/commits-since/shindler7/dj/latest)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

# Dj — a console utility for running Django applications

At some point, you might ask yourself: why not use uv and tuna to run Django
applications in Python? No problem there. But soon enough, you find yourself
juggling a growing number of command-line parameters that all need to be typed
in manually.

You could wrap it all in a .sh script, but that means dealing with unfamiliar,
error-prone syntax, poor readability, and little room for validation. This
application takes care of the heavy lifting, handling all the command-line
boilerplate so you don't have to.

## Built with Rust

This little tool is written in Rust — because why not bring some speed and
reliability to the party? It's a small personal project, but it's sitting right
there on GitHub, open for anyone to use, tweak, or just poke around. If it
saves you a few keystrokes too, feel free to grab it and make it your own.

## Quick Start

Make sure you have `Rust` 1.93 or later installed on your system.

### 1. Clone the repository

```shell
git clone git@github.com:Shindler7/dj.git
cd dj/
```

### 2. Build from source

```shell
cargo build --release
```

The binary will be available at `target/release/dj`.

### 3. Install globally (optional)

If you want to use `dj` from anywhere on your system:

```shell
cargo install --path .
```

This will install the dj binary to your Cargo bin directory (usually
`~/.cargo/bin`). Make sure it's in your `PATH`.

### 4. Set up your project

Create a `start.toml` file in your Django project root. Check out the
Configuration section for a complete example.

> ⚠️ **Important:** Never hardcode sensitive values (API keys, passwords, etc.)
> directly in `start.toml`. Instead, use a `.env` file for your secrets and
> reference them with `${VAR}` placeholders — for example,
> `api_key = "${TUNA_API_KEY}"`. This keeps your credentials out of version
> control.

### 5. Run your Django app

```shell
# Start the development server (default)
dj runserver

# or just use the alias
dj run

# Run any Django management command
dj migrate
dj shell
dj test

# Short aliases for common commands
dj m    # migrate
dj mm   # makemigrations
dj s    # shell
```

That's it. Go build something awesome!

## Command-line Interface

```shell
dj [COMMAND] [ARGS]...
```

| Command                         | Description                                                       |
|:--------------------------------|:------------------------------------------------------------------|
| `dj runserver` / `dj run`       | Start the Django development server (default)                     |
| `dj <command>`                  | Proxy any command to python manage.py <command>                   |
| `dj example <SCRIPT> [ARGS]...` | Run a custom Python script with the same environment and features |

## Configuration

Dj looks for a `start.toml` file in the **current working directory**. Check
out the [example configuration](start.toml) for all available options.

All sections are optional — Dj uses sensible defaults for everything.

## Features

### Tuna integration

When `features.tuna = true`, Dj wraps your command with `tuna secrets run`,
injecting cloud secrets into your environment. Perfect for development
environments that depend on external secrets.

### uv support

When `features.uv = true`, Dj uses `uv run` instead of the default Python
interpreter. Faster dependency resolution and better package management — all
with zero extra config.

### Example mode

The `example` command lets you run any Python script with the same environment
and feature wrappers as your Django app. It's ideal for:

- Debugging scripts
- One-off data migrations
- Testing code that depends on Tuna secrets or `uv`

**Example:**

```shell
dj example scripts/hello.py
dj example scripts/debug.py --verbose
```

## Known Issues

### Graceful shutdown for async commands

`Dj` doesn't yet handle graceful shutdown correctly when running asynchronous
commands — for example, a `uvicorn` server with `--reload`:

```toml
[django]
runserver = ["python", "-m", "uvicorn", "--reload", "rustorgpred.asgi:application", "--port", "8080"]
```

#### What happens

When you press `Ctrl+C`, the child process begins shutting down, but the
terminal output gets garbled — log lines from the child and the parent
interleave, and the console prompt returns before the process has fully exited.

What we'd expect: `Dj` should wait for the child to finish its shutdown
sequence and return control only after the terminal is in a clean state.

#### Why it's tricky

Asynchronous servers like `uvicorn` install their own signal handlers and spawn
a reloader parent + worker child. The signal reaches Dj first, and by the time
the child handles it, the terminal has already been partially handed back.

Synchronous commands (`runserver`, `shell`, `migrate`, etc.) work fine — the
issue only affects commands that manage their own signals and subprocesses.

#### Status

**Open** — no idiomatic solution found yet. Suggestions and PRs are welcome.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for release history and version details.
