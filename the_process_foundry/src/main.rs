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
pub mod error;
pub mod registry;
pub mod shell;

use registry::*;

// Get a bash object
pub fn get_bash(registry: &Registry) -> Result<Box<dyn AppTrait>> {
    let def = AppDefinition {
        name: "Bash".to_string(),
        ..Default::default()
    };

    let matches = registry.find(&def)?;
    let factory = match matches.len() {
        0 => Err(FoundryError::NotFound).context(format!(
            "No instances matching your defition of {} have been registered",
            def.name
        ))?,
        1 => matches.get(0).unwrap(),
        x => Err(FoundryError::MultipleMatches).context(format!(
            "{} instances matching your defition for {} have been registered. Please narrow your criteria",
            x, def.name
        ))?,
    };
    factory.build()
}

pub fn bootstrap() -> Result<()> {
    // Bootstrap
    // - Create Registry
    let mut registry = Registry::new();
    log::debug!("The registry has been created:\n{}", registry);

    // - Load the core Apps
    registry.register_factory(Box::new(applications::BashFactory::new()))?;
    log::debug!("I've registered Bash:\n{}", registry);

    // - Get the local shell from foundry - default user shell, fallback bash
    let local = get_bash(&registry)?;

    // - Run "echo Hello World" on bash

    // - Find docker-compose on bash

    // - Register docker-compose app in registry

    // - Get docker-compose app from foundry

    // - Get docker-compose container from compose app

    // - Read yaml into docker-compose

    // - Register postgres with registry

    // - Get postgres app from compose container?

    // - Register pg_basebackup with foundry
    // - Get pg_basebackup app based on compose/postgres app

    // - Refactor and rename - test and document things so they make sense
    Ok(())
}

fn main() {
    env_logger::init();
    log::info!("Starting to run the Process foundry");
    let _ = bootstrap().unwrap();
    log::info!("Finished bootstrapping the foundry")
}
