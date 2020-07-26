//! A wrapper around external software to generate TPF Actions and Events
//!
//! An application (app) is an interface which defines a set of functions to convert TPF actions into native
//! commands, monitor state, and sends out messages describing any changes in state.
//!
//! At its core, an application is something that executes actions and emits events. It has no context of
//! anything outside the scope of its functionality. It can be accessed by one or more containers, depending
//! on the context.
//!
//! For more detailed info about the architecture, read
//! https://lurkingfrog.github.io/the_process_foundry_book/core/application.html

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::rc::Rc;

use super::FoundryError;
use super::{AppQuery, ContainerTrait};

/// A **data structure** containing information used to generate both generate Actions and Events
///
///
pub trait AppTrait: std::fmt::Debug {
  /// Construct the metadata for the module controlling the app instance
  /// This is important to make sure the module aligns with the actual installed instance
  fn new(instance: AppInstance, parent: Option<Rc<dyn ContainerTrait>>) -> Result<Self>
  where
    Self: Sized;

  /// Print a standardized "name (version)" for logging purposes
  fn get_name(&self) -> String;

  /// Knows how to get the version number of the installed app (not the module version)
  fn set_version(&self, instance: AppInstance) -> Result<AppInstance> {
    log::warn!(
      "Set version has not been implemented for {}",
      self.get_name()
    );
    Ok(instance)
  }

  // /// Figures out how to call the cli using the given container
  // /// THINK: is it better to have an option or make "Local" a special case?
  // fn set_cli(
  //   &self,
  //   instance: AppInstance,
  //   container: Rc<dyn ContainerTrait>,
  // ) -> Result<AppInstance>;

  // list_actions
}

/// General information about the external application handled by this particular module
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize)]
pub struct AppDescription {
  /// The name the external application is known by
  name: String,

  /// The versions of the external app that are handled by this App
  handles_versions: Option<semver::VersionReq>,

  // ****  Some query helper information  *** //
  /// Names that this app might also be known as
  ///
  /// # Examples: postgres may also be known as `pg` or `postgresql`
  aliases: Option<Vec<String>>,

  /// Some standard paths to look for information
  search_paths: Option<Vec<String>>,
  // Network polling? ports, process name, remote_ip?
}

impl std::fmt::Display for AppDescription {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:#?}", self)
  }
}

impl AppDescription {
  /// Use the description to create a query to find running instances of this app
  pub fn to_app_query(&self) -> AppQuery {
    AppQuery {
      name: self.name.clone(),
      works_with: self.handles_versions.clone(),
      aliases: self.aliases.clone(),
      search_paths: self.search_paths.clone(),
      ..Default::default()
    }
  }
}

/// Information about the specific running copy of the external program
///
/// This is a synthesis of both introspected information (version) and external (ip address/port)
/// THINK: Should this identify the environment?
/// THINK: Should the app instance know itself/functions?
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AppInstance {
  /// This is for use by Containers enumerating instances of their content
  // THINK: Use hash value of instance? https://doc.rust-lang.org/std/hash/index.html#examples
  pub instance_id: Option<String>,

  /// The standard name for this app (e.g. Postgres, DockerCompose)
  pub name: String,

  /// The version of the installed app
  /// NOTE: Some items (like sh), don't easily have a version available, so we make it optional
  pub version: Option<semver::Version>,

  /// The version of the Foundry code running
  /// NOTE: This will be much more important once TPF becomes distributed microservices
  pub module_version: Option<semver::Version>,

  pub config_file: Option<String>,

  // pub codebase: Option<Box<dyn AppTrait>>,
  pub cli: Option<CliAccess>,
  pub api: Option<ApiAccess>,
}

impl AppInstance {
  pub fn new(name: String) -> AppInstance {
    AppInstance {
      name,
      ..Default::default()
    }
  }

  pub fn full_name(&self) -> String {
    match &self.version {
      Some(ver) => format!("{} ({})", self.name, ver),
      None => format!("{} ({})", self.name, "Unknown Version"),
    }
  }

  pub fn set_command_path(
    &self,
    container: Option<Rc<dyn ContainerTrait>>,
    path: String,
  ) -> Result<AppInstance> {
    let cli = self.cli.clone().map_or(
      CliAccess {
        path: path.clone(),
        container: container.clone(),
      },
      |cli| CliAccess {
        path: path.clone(),
        ..cli.clone()
      },
    );

    Ok(AppInstance {
      cli: Some(cli),
      ..self.clone()
    })
  }

  pub fn get_command_path(&self) -> Result<String> {
    match &self.cli {
      None => Err(FoundryError::NotConfigured).context(format!("Cli is not set for {}", self.name)),
      Some(cli) => Ok(cli.path.clone()),
    }
  }
}

impl std::fmt::Debug for AppInstance {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "AppInstance {{
        name: {},
        version: {:#?},
        config_file: {:#?},
        cli: {}
        network: {}
      }}",
      self.name,
      self.version,
      self.config_file,
      self
        .cli
        .clone()
        .map_or("None".to_string(), |cli| cli.path.clone()),
      self.api.clone().map_or("None", |_net| {
        "Networking not implemented yet for AppInstance"
      }),
    )
  }
}

impl std::fmt::Display for AppInstance {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "AppInstance {} ({:#?})", self.name, self.version)
  }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CliAccess {
  /// The container where this App is found
  #[serde(skip)]
  pub container: Option<Rc<dyn ContainerTrait>>,

  /// The location of the executable
  pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiAccess {
  pub uri: String,
}
