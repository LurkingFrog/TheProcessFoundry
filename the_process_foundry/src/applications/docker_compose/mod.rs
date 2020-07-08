//! Manage the docker-compose application
//!
//! This should be able to load a config from a file, run commands based on the config, and
//! then write any changes back to a file.
//!
//! I'm writing this because the existing parser only parsed V2 files and there didn't appear to be any
//! easy way to add in V3 functionality without breaking things for existing users.
//!
//! TODO: Figure out how to scan to guess the process being run.

pub mod application;
pub mod schema;

use super::*;
pub use application::{Action, ActionResult, DockerCompose, Event};
