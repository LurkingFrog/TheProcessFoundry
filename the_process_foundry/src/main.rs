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

use applications::Bash;
use base::*;

//
// Get a bash object
// pub fn find_bash(registry: &Registry) -> Result<Box<dyn AppTrait>> {
//     let def = AppQuery {
//         name: "Bash".to_string(),
//         ..Default::default()
//     };

//     let matches = registry.find(&def)?;
//     let factory = match matches.len() {
//         0 => Err(FoundryError::NotFound).context(format!(
//             "No instances matching your defition of {} have been registered",
//             def.name
//         ))?,
//         1 => matches.get(0).unwrap(),
//         x => Err(FoundryError::MultipleMatches).context(format!(
//             "{} instances matching your defition for {} have been registered. Please narrow your criteria",
//             x, def.name
//         ))?,
//     };

//     factory.build(None)
// }

// pub fn find_docker_compose(shell: Box<dyn ContainerTrait>) -> Result<DockerCompose> {
//     let compose_query = AppQuery::new("docker-compose".to_string()).unique();
//     let action = shell
//         .find_app(compose_query)
//         .context("Could not find docker-compose")?;
//     // let inst = action.run(bash, compose)?;
// }

pub fn bootstrap() -> Result<()> {
    // Get the local bash shell
    let shell = base::Shell::get_local_shell()
        .context("Oh noes, my bootstrap failed to get a local shell")?;

    let bash = Bash::build(shell.instance.clone())
        .context("Building bash from the local shell didn't work")?;

    // Find Docker Compose using local bash
    // let dc = find_docker_compose(bash).context("Couldn't bootstrap docker compose")?;

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
