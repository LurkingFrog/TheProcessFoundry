//! The main library of the Process Foundry
//!
//! This is here to bootstrap and tie the entire system together.
//!
//! # Some Definitions
//!
//! ## Application - An item which can run actions and generate events
//! ## Container - An abstract object which contains applications and other containers. It can be used to look
//!    its own contents up and pass actions to them.
//! ## Module - An implementation of an interface to an Application
//! ## Registry - This a singleton that maintains a searchable list of application and container factories
//! ## Workflow - A set of Applications wired together to implement a single process

use anyhow::{Context, Result};
use error::FoundryError;

pub mod applications;
pub mod base;
pub mod error;
// pub mod helpers;
// pub mod registry;

use applications::{Bash, DockerCompose};
use base::*;

pub fn find_docker_compose(shell: Box<dyn ContainerTrait>) -> Result<DockerCompose> {
    let compose_query = AppQuery::new("docker-compose".to_string());
    let instance = shell
        .find_app(compose_query)
        .context("Could not find docker-compose")?;
    DockerCompose::build(instance.get(1).unwrap().clone())
}

pub fn bootstrap() -> Result<()> {
    // Get the local bash shell
    let shell = base::Shell::get_local_shell()
        .context("Oh noes, my bootstrap failed to get a local shell")?;

    let bash = Bash::build(shell.instance.clone())
        .context("Building bash from the local shell didn't work")?;

    // Find Docker Compose using local bash
    let dc = find_docker_compose(Box::new(bash)).context("Couldn't bootstrap docker compose")?;

    // Load Docker Compose

    // Get Postgres Docker

    // Find PG Backup on Postgres

    Ok(())
}

fn main() {
    env_logger::init();
    log::info!("Starting to run the Process foundry");

    let _ = bootstrap().unwrap();
    log::info!("Finished bootstrapping the foundry")
}
