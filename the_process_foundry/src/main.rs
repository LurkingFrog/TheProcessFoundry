//! The main library of the Process Foundry
//!
//! This is here to bootstrap and tie the entire system together.

use anyhow::Result;
use error::FoundryError;

pub mod applications;
pub mod error;
pub mod registry;

use registry::*;

pub fn bootstrap() -> Result<()> {
    // Bootstrap
    // - Create Registry
    let mut registry = Registry::new();
    log::debug!("The registry has been created:\n{}", registry);

    // - Load the core Apps
    registry.register_factory(Box::new(applications::BashFactory::new()))?;
    log::debug!("I've registered Bash:\n{}", registry);

    // - Get the local shell from foundry - default user shell, fallback bash
    // - Get docker-compose app from foundry
    // - Get docker-compose container from compose app
    // - Get postgres app from compose container?
    // - Get pg_basebackup app based on compose/postgres app
    Ok(())
}

fn main() {
    env_logger::init();
    log::info!("Starting to run the Process foundry");
    let _ = bootstrap();
    log::info!("Finished bootstrapping the foundry")
}
