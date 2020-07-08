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

use applications::{Bash, DockerCompose, DockerContainer};
use base::*;

pub fn find(container: Box<dyn ContainerTrait>, app_name: String) -> Result<AppInstance> {
    let query = AppQuery::new(app_name.clone());
    let instance = container
        .find_one(query)
        .context(format!("Could not find the {} in bootstrap", app_name))?;
    Ok(instance)
}

pub fn bootstrap() -> Result<()> {
    // Get the local bash shell
    let shell = base::Shell::get_local_shell()
        .context("Oh noes, my bootstrap failed to get a local shell")?;

    let bash = Box::new(
        Bash::build(shell.instance.clone())
            .context("Building bash from the local shell didn't work")?,
    );

    // Find Docker Compose using local bash and load the test compose file
    let dc = Box::new(
        DockerCompose::build(
            find(bash.clone(), "docker-compose".to_string())?
        )?
        .load("/home/dfogelson/Foundry/TheProcessFoundry/the_process_foundry/tests/data/postgres.docker-compose.yml".to_string())?);

    // Find postgres container
    let pg = find(dc.clone(), "postgres".to_string())?;
    let pg_container = Box::new(dc.get_container(pg.name)?);

    // Find PG Backup on Postgres
    // let pg_backup = find(pg_container.clone(), "pg_backup".to_string())?;

    // Get the local filesystem
    // let fs = fs::FileSystem(bash)
    Ok(())
}

fn main() {
    env_logger::init();
    log::info!("Starting to run the Process foundry");

    let _ = bootstrap().unwrap();
    log::info!("Finished bootstrapping the foundry")
}
