//! The core definitions and structs for the Process Foundry
//!
//! After getting lost in the weeds, this is all the things that I seem to see come up as useful in every
//! application and container
//!
//! # THINK: Additional traits
//!   - Actionable: Possibly part of app trait, since all should be able to utilize and emit Actions/Events
//!   - ActionResult: To abstract the result so we can pass it to something that has the code to actually use
//!   - Routable (possibly part of action)
//!   - consider https://abronan.com/rust-trait-objects-box-and-rc/ - Arc<Mutex<impl trait>>

use anyhow::{Context, Result};
use serde_derive::{Deserialize, Serialize};

use super::FoundryError;

// Re-exports

pub mod application;
#[doc(inline)]
pub use application::*;

pub mod container;
#[doc(inline)]
pub use container::*;

pub mod message;
#[doc(inline)]
pub use message::*;

pub mod shell;
#[doc(inline)]
pub use shell::*;

/// An app running on the local system (eg: bash shell)
// pub trait LocalTrait {
//   /// Get a local instance of the given app
//   fn get_local() -> Result<AppInstance>;
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliQueryOptions {
  pub search_paths: Option<Vec<String>>,
}

/// A wrapper for looking for specific instances of apps or where they are running
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryOptions {
  Container,
  Network,
  Cli,
}

/// This defines the a needed version of container/application needed for the module.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppQuery {
  // Main options
  pub name: String,
  pub works_with: Option<semver::VersionReq>,

  pub aliases: Option<Vec<String>>,
  pub search_paths: Option<Vec<String>>,

  // Searches high and low for apps that match the criteria. If false it returns an error with multiple matches.
  pub find_all: bool,
}

impl AppQuery {
  pub fn new(name: String) -> AppQuery {
    AppQuery {
      name,
      works_with: None,

      aliases: None,
      search_paths: None,
      find_all: false,
    }
  }

  // TODO: this seems to make sense
  // pub fn new2(def: AppDescription) -> AppQuery {}

  // Set find all option to true
  pub fn find_all(&self) -> AppQuery {
    AppQuery {
      find_all: true,
      ..self.clone()
    }
  }
}

/// Handlers for serialized action requests
///
/// THINK: Should there be a send/receive?
pub trait ActionTrait {
  type RESPONSE;

  // Have the application directly run the function run the command and return the result
  fn run(&self, target: AppInstance) -> Result<Self::RESPONSE>;

  // Convert this action into a std::process::Command style vector to be run in a place where the
  // foundry cannot directly access (like inside docker container)
  // THINK: Should run just naturally use this when the target is remote?
  fn to_message(&self, target: Option<AppInstance>) -> Result<Vec<Message>>;

  // This is a long term goal, be able to generate stand-alone scripts based on the container actions
  // fn to_file(&self, application: Box<dyn AppTrait>) -> Result<String, Error>;
}
