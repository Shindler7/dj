//! Core data structures used throughout the application.

use anyhow::{Context, Result as AnyhowResult};
use serde::Deserialize;
use std::{
    ops::{Deref, DerefMut},
    process::Command,
};

/// Container for additional command-line arguments to pass through.
#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(from = "Vec<String>")]
pub(crate) struct DjangoCommands {
    command: Vec<String>,
}

impl From<Vec<String>> for DjangoCommands {
    fn from(command: Vec<String>) -> Self {
        Self { command }
    }
}

impl From<Vec<&str>> for DjangoCommands {
    fn from(command: Vec<&str>) -> Self {
        Self {
            command: command.into_iter().map(ToString::to_string).collect(),
        }
    }
}

impl TryFrom<DjangoCommands> for Command {
    type Error = anyhow::Error;

    fn try_from(dj_commands: DjangoCommands) -> AnyhowResult<Self> {
        let mut parts = dj_commands.command.into_iter();
        let program = parts.next().context("Django command is empty")?;

        let mut command = Command::new(&program);
        command.args(parts);
        Ok(command)
    }
}

impl IntoIterator for DjangoCommands {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.command.into_iter()
    }
}

impl Deref for DjangoCommands {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.command
    }
}

impl DerefMut for DjangoCommands {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.command
    }
}

impl DjangoCommands {
    // Creates an empty list of Django commands.
    pub(crate) fn new() -> Self {
        Self {
            command: Vec::new(),
        }
    }
}
