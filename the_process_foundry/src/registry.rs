//! The lookup registry for the Rust Objects doing the work
//!

use super::FoundryError;
use anyhow::Result;

use std::collections::HashMap;

/// This is the base index of the full system.
///
/// Additional features to come after the initial use case (Backup Postgres) works.
/// - Locking/Mutex on specific actions
/// - Search/Find features based on the ApplicationDefinition
/// - Ability to add a generic DLL factory so we don't need to have the whole thing at compile time
#[derive(Default)]
pub struct Registry {
  factory_registry: HashMap<String, Box<dyn FactoryTrait>>,
}

impl std::fmt::Display for Registry {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "(Registry knows about {} factories loaded)",
      self.factory_registry.keys().len()
    )
  }
}

impl Registry {
  pub fn new() -> Registry {
    Default::default()
  }

  // /// Add a AppFactory definition as available to the Process Foundry
  pub fn register_factory(&mut self, app: Box<dyn FactoryTrait>) -> Result<(), FoundryError> {
    unimplemented!("Cannot register factories yet")
  }
}

trait FactoryTrait {
  fn get_definition(&self) -> ApplicationDefinition;
}

/// Most items act as both an Application and a Container, depending upon the context. Each will need
/// both traits, I need a way to tell which one is actually available at run time.
///
/// More research into traits/peer review will expose this
#[derive(Debug, Clone)]
pub enum ActsAs {
  Container,
  Application,
  Either,
}

/// This defines the version of container/application management code.
#[derive(Debug, Clone)]
pub struct ApplicationDefinition {
  pub name: String,
  pub acts_as: ActsAs,
  pub app_version: Option<semver::Version>,
  pub works_with: Option<semver::VersionReq>,
  pub aliases: Option<Vec<String>>,
}

impl ApplicationDefinition {
  pub fn new(name: String) -> ApplicationDefinition {
    ApplicationDefinition {
      name,
      acts_as: ActsAs::Either,
      app_version: None,
      works_with: None,
      aliases: None,
    }
  }
}
