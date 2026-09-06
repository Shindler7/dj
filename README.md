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

> ⚠️ Important: Never hardcode sensitive values (API keys, passwords, etc.)
> directly in
> start.toml. Instead, use a .env file for your secrets and reference them
> with ${VAR} placeholders — for example, api_key = "${TUNA_API_KEY}". This
> keeps your
> credentials out of version control.

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

| Command                          | Description                                                       |
|:---------------------------------|:------------------------------------------------------------------|
| `dj runserver` / `dj run`        | Start the Django development server (default)                     |
| `dj <command>`                   | Proxy any command to python manage.py <command>                   |
| `dj example <SCRIPTS> [ARGS]...` | Run a custom Python script with the same environment and features |

## Configuration

Dj looks for a `start.toml` file in the current directory. Check out the
[example configuration](start.toml) for all available options.

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

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for release history and version details.
