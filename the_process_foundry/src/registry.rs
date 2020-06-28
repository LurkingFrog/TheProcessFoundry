//! The lookup registry for the Rust Objects doing the work
//!

use anyhow::{Context, Result};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

use super::FoundryError;

/// This is the base index of the full system.
///
/// Main TODOs:
/// - Convert the keys from keys to Uuids
///
/// Additional features to come after the initial use case (Backup Postgres) works.
/// - Locking/Mutex on specific actions
/// - Search/Find features based on the AppDefinition
/// - Ability to add a generic DLL factory so we don't need to have the whole thing at compile time
#[derive(Default)]
pub struct Registry {
  factories: HashMap<String, Box<dyn FactoryTrait>>,
  actions: HashMap<String, AppDefinition>,
  events: HashMap<String, AppDefinition>,
}

impl std::fmt::Display for Registry {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "Registry has {} factories loaded, allowing {} types of action and producting {} types of event",
      self.factories.keys().len(), self.actions.keys().len(), self.events.keys().len()
    )
  }
}

impl Registry {
  pub fn new() -> Registry {
    Default::default()
  }

  /// Add a AppFactory definition as available to the Process Foundry
  /// TODO: Either create a better key than just name or add lookup elements to the registry (likely both)
  pub fn register_factory(&mut self, app: Box<dyn FactoryTrait>) -> Result<()> {
    let def = app.get_definition()?;
    log::info!("Registering new factory: {}", def.full_name());
    // Collision if already exists
    match self.factories.get(&def.name) {
      Some(_) => Err(FoundryError::DuplicateKeyError).context(format!(
        "An application named '{}' has already been registered",
        def.full_name()
      ))?,
      None => self.factories.insert(def.name.clone(), app),
    };
    Ok(())
  }

  /// Find all the applications avaliable that match the given definition
  /// TODO: this needs to be much more granular than just name
  pub fn find(&self, def: &AppDefinition) -> Result<Vec<&Box<dyn FactoryTrait>>> {
    match self.factories.get(&def.name) {
      Some(factory) => Ok(vec![factory.clone()]),
      None => Ok(vec![]),
    }
  }
}

/// Allow an instance of the particular application to be created on demand. This exposes a serialized
/// API of the App. While this allows us to scale horizontally, access using a direct import
/// will benchmark faster.
pub trait FactoryTrait {
  fn get_definition(&self) -> Result<AppDefinition>;
  fn build(&self) -> Result<Box<dyn AppTrait>>;
}

/// Most items act as both an App and a Container, depending upon the context. Each will need
/// both traits, I need a way to tell which one is actually available at run time.
///
/// More research into traits/peer review will expose this
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActsAs {
  Container,
  App,
  Either,
}

impl Default for ActsAs {
  fn default() -> Self {
    ActsAs::Either
  }
}

/// This defines the version of container/application management code as well as used for searching the
/// registry
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppDefinition {
  pub name: String,
  pub module_version: Option<semver::Version>,
  pub acts_as: ActsAs,
  pub works_with: Option<semver::VersionReq>,
  pub aliases: Option<Vec<String>>,
}

impl AppDefinition {
  pub fn new(name: String, version: semver::Version) -> AppDefinition {
    AppDefinition {
      name,
      acts_as: ActsAs::Either,
      module_version: Some(version),
      works_with: None,
      aliases: None,
    }
  }

  pub fn full_name(&self) -> String {
    match &self.module_version {
      Some(ver) => format!("{} ({})", self.name, ver),
      None => format!("{} ({})", self.name, "Unknown Version"),
    }
  }
}

// A pointer to an Application found in a container
/// Describe a specific version of an application
/// TODO: Should this identify the environment?
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppInstance {
  /// This is for use by Containers enumerating instances of their content
  pub process_id: Option<String>,

  /// The standard name for this app (e.g. Postgres, DockerCompose)
  pub name: String,

  /// The path of the executable
  pub path: Option<String>,

  /// Version of the app that the factory returns
  pub version: semver::Version,

  /// An ID used to look up the specific Application based on the instance
  pub factory_id: Option<uuid::Uuid>,

  /// The container where this App is found
  pub container_id: Option<uuid::Uuid>,
}

/// The interface for running actions on an application
pub trait AppTrait {
  fn get_name(&self) -> String;
  // fn find(&self, definition: AppDefinition) -> Result<AppInstance>;
  // fn act(&self, action: Box<dyn ActionTrait>) -> Result<()>;
}

pub trait ContainerTrait {}

pub trait ActionTrait {
  type RESPONSE;

  // Have the application directly run the function run the command and return the result
  fn run(&self, container: Box<dyn AppTrait>, application: AppDefinition)
    -> Result<Self::RESPONSE>;

  // Convert this action into a std::process::Command style vector to be run in a place where the
  // foundry cannot directly access (like a docker container)
  fn to_string(&self, application: Box<dyn AppTrait>) -> Result<Vec<String>>;

  // This is a long term goal, be able to generate stand-alone scripts based on the container actions
  // fn to_file(&self, application: Box<dyn AppTrait>) -> Result<String, Error>;
}
