mod command;
mod directory;
mod file;
mod package;

use crate::manifests::Manifest;
use anyhow::Result;
use command::run::CommandRun;
use directory::copy::DirectoryCopy;
use file::copy::FileCopy;
use file::link::FileLink;
use package::install::PackageInstall;
use serde::{Deserialize, Serialize};
use tera::Context;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "action")]
pub enum Actions {
    #[serde(alias = "command.run", alias = "cmd.run")]
    CommandRun(CommandRun),
    #[serde(alias = "directory.copy", alias = "dir.copy")]
    DirectoryCopy(DirectoryCopy),
    #[serde(alias = "file.copy")]
    FileCopy(FileCopy),
    #[serde(alias = "file.link")]
    FileLink(FileLink),
    #[serde(alias = "package.install", alias = "package.installed")]
    PackageInstall(PackageInstall),
}

impl Actions {
    pub fn inner_ref(&self) -> &dyn Action {
        match self {
            Actions::CommandRun(a) => a,
            Actions::DirectoryCopy(a) => a,
            Actions::FileCopy(a) => a,
            Actions::FileLink(a) => a,
            Actions::PackageInstall(a) => a,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActionResult {
    /// Output / response
    pub message: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActionError {
    /// Error message
    pub message: String,
}

impl<E: std::error::Error> From<E> for ActionError {
    fn from(e: E) -> Self {
        ActionError {
            message: format!("{}", e),
        }
    }
}

pub trait Action {
    fn run(&self, manifest: &Manifest, context: &Context) -> Result<ActionResult>;

    fn dry_run(&self, manifest: &Manifest, context: &Context) -> Result<ActionResult>;
    fn changeset(&self, manifest: &Manifest, _context: &Context) -> Option<ChangeSet> {
        Some(ChangeSet {
            changes: vec![Box::new(NoOp)],
        })
    }
}

pub trait Atom: std::fmt::Debug {
    fn apply(&self, ctx: &Context) -> Result<()>; // Apply new to old
    fn revert(&self); // Revert new to old
    fn diff(&self); // Print a string that describes what it will do. Show old and new values
}

#[derive(Debug)]
struct NoOp;

impl Atom for NoOp {
    fn apply(&self, _ctx: &Context) -> Result<()> {
        Ok(())
    }

    fn revert(&self) {}

    fn diff(&self) {}
}

#[derive(Debug)]
pub struct ChangeSet {
    changes: Vec<Box<dyn Atom>>,
}
