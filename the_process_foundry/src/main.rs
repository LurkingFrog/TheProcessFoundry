//! The root library of the Process Foundry
//!
//! This is here to bootstrap and tie the entire system together.
//!
//! TODO: Make more of a README for this
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
use std::rc::Rc;

pub mod applications;
pub mod base;
pub mod error;
// pub mod helpers;
// pub mod registry;

// use applications::{Bash, DockerCompose, PgBaseBackup};

#[doc(inline)]
pub use base::*;

pub fn find(container: Rc<dyn ContainerTrait>, app_name: String) -> Result<AppInstance> {
    let query = AppQuery::new(app_name.clone());
    let instance = container
        .find_one(query)
        .context(format!("Could not find {} in bootstrap", app_name))?;
    Ok(instance)
}

pub fn bootstrap() -> Result<()> {
    // Get the local bash shell
    let shell =
        Shell::get_local_shell().context("Oh noes, my bootstrap failed to get a local shell")?;

    // let bash = Rc::new(
    //     Bash::new(shell.instance.clone(), None)
    //         .context("Building bash from the local shell didn't work")?,
    // );

    // // Find Docker Compose using local bash and load the test compose file
    // let dc = Rc::new(
    //     DockerCompose::new(
    //         find(bash.clone(), "docker-compose".to_string())?,
    //         Some(bash.clone())
    //     )?
    //     .load("/home/dfogelson/Foundry/TheProcessFoundry/the_process_foundry/tests/data/postgres.docker-compose.yml".to_string())?);

    // // TODO: Does the container have a mounted volume named backup? (Only implementing "yes" for now)

    // // Find postgres container
    // let pg_service = find(dc.clone(), "postgres".to_string())?;
    // let pg_container = Rc::new(dc.get_container(pg_service.name)?);

    // // TODO: Is the container running? Start if not

    // // Find PG Backup on Postgres
    // let pg_backup = PgBaseBackup::new(
    //     find(pg_container.clone(), "pg_basebackup".to_string())?,
    //     Some(pg_container.clone()),
    // )?;

    // let options = applications::pg_basebackup::Options::new("/backup".to_string());
    // pg_backup.run(options).context("Could not make a backup")?;

    Ok(())
}

fn main() {
    env_logger::init();
    log::info!("Starting to run the Process foundry");

    // Convert "bootstrap" into workflow ()
    // Change "Bash" to "Shell" with a language type of Bash
    // Make containers Async
    // Register each type of container/app factory?
    // Make instance registry for routing - Container should have "register_known_children" function
    // Make workflow for postgres backup
    // Make listeners/emitters
    // Add router to pass messages (Events/Actions?)

    match bootstrap() {
        Ok(_) => log::info!("Finished running the bootstrap without errors"),
        Err(err) => log::error!("Failed to complete bootstrap because of error:\n{}", err),
    }
    log::info!("Finished bootstrapping the foundry")
}
