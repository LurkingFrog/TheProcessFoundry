//! The main library of the Process Foundry
//!
//! This is here to bootstrap and tie the entire system together.

use anyhow::Result;
use error::FoundryError;

pub mod error;
pub mod registry;

use registry::Registry;

pub fn create_registry() -> Result<Registry> {
    unimplemented!("No Create for you")
}

pub fn bootstrap() -> Result<()> {
    // Bootstrap
    // - Create Registry
    let registry = create_registry()?;

    // - Load the core Apps
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
}
