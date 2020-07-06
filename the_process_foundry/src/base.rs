//! The core definitions and structs for the Process Foundry
//!
//! After getting lost in the weeds, this is all the things that I seem to see come up as useful in every
//! application and conatainer

use super::Bash;
use anyhow::{Context, Result};
use serde_derive::{Deserialize, Serialize};

/// All applications should be introspective and able to build a running manager of itself
pub trait AppTrait {
  /// Construct the metadata for the module controlling the app instance
  /// This is important to make sure the module aligns with the actual installed instance
  fn build(instance: AppInstance) -> Result<Self>
  where
    Self: Sized;

  /// Print a standardized "name (version)" for logging purposes
  fn get_name(&self) -> String;

  /// Knows how to get the version number of the installed app (not the module version)
  fn set_version(instance: AppInstance) -> Result<AppInstance>;

  /// Figures out how to call the cli using the given container
  /// THINK: is it better to have an option or make "Local" a special case?
  fn set_cli(instance: AppInstance, container: Box<dyn ContainerTrait>) -> Result<AppInstance>;
}

/// Ways to manage applications (eg Docker, Bash) contained within itself
pub trait ContainerTrait {
  /// This will find a list of apps with configurations that the container knows about
  fn find_app(&self, query: AppQuery) -> Result<Vec<AppInstance>>;

  /// List the known items in the app cache
  fn found_apps(&self) -> Result<Vec<AppInstance>>;

  /// Get the name/version of the container, usually for use in logging/errors.
  fn get_name(&self) -> String;
}

/// An app running on the local system (eg: bash shell)
pub trait LocalTrait {
  /// Get a local instance of the given app
  fn get_local() -> Result<AppInstance>;
}

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

  // Searches high and low for apps that match the criteria. If false (default) it returns only the first item found
  pub find_all: bool,
}

impl AppQuery {
  pub fn new(name: String) -> AppQuery {
    AppQuery {
      name,
      works_with: None,

      aliases: None,
      find_all: false,
    }
  }

  // Set find all option to true
  pub fn find_all(&self) -> AppQuery {
    AppQuery {
      find_all: true,
      ..self.clone()
    }
  }
}

/// A description of all the known information about discovered application.
///
/// This is a synthesis of both introspected information (version) and external (ip address/port)
/// THINK: Should this identify the environment?
/// THINK: Should the app instance know itself/functions?
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppInstance {
  /// This is for use by Containers enumerating instances of their content
  pub process_id: Option<String>,

  /// The standard name for this app (e.g. Postgres, DockerCompose)
  pub name: String,

  /// The version of the installed app
  /// NOTE: Some items (like sh), don't easily have a version available, so we make it optional
  pub version: Option<semver::Version>,

  /// The version of the Foundry code running
  /// NOTE: This will be much more important once TPF becomes distributed microservices
  pub module_version: Option<semver::Version>,

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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliAccess {
  /// The container where this App is found
  pub container_id: Option<uuid::Uuid>,

  /// The location of the executable
  pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiAccess {
  pub uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShellType {
  Dash,
  Bash,
  Zsh,
}

/// A special case for bootstrapping. I'm trying to find the enumerations that actually deserve to be
/// traits themselves
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shell {
  pub shell_type: ShellType,
  pub instance: AppInstance,
}

impl Shell {
  pub fn get_local_shell() -> Result<Shell> {
    // Bash seems to be on most systems, so we'll prefer that
    let default = "bash";
    let shell_name = option_env!("SHELL").map_or(default, |cmd| {
      let regex = regex::Regex::new(r"/([\w-]+)$").unwrap();
      regex.captures(&cmd).map_or(default, |cap| {
        cap.get(1).map_or(default, |val| val.as_str())
      })
    });

    let shell_type = match &shell_name.to_lowercase()[..] {
      "bash" => ShellType::Bash,
      x => {
        log::warn!("Found preferred shell is '{}', but using bash anyways", x);
        ShellType::Bash
      }
    };

    let instance = match shell_type {
      ShellType::Bash => Bash::get_local()?,
      _ => unreachable!("Should currently not be able to use any local shell other than bash"),
    };

    Ok(Shell {
      instance,
      shell_type,
    })
  }
}

pub trait ActionTrait {
  type RESPONSE;

  // Have the application directly run the function run the command and return the result
  fn run(&self, target: AppInstance) -> Result<Self::RESPONSE>;

  // Convert this action into a std::process::Command style vector to be run in a place where the
  // foundry cannot directly access (like a docker container)
  fn to_string(&self, target: AppInstance) -> Result<Vec<String>>;

  // This is a long term goal, be able to generate stand-alone scripts based on the container actions
  // fn to_file(&self, application: Box<dyn AppTrait>) -> Result<String, Error>;
}
