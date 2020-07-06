//! Docker-Compose functionality
//!
//! A separate app to examine and run docker compose

const APP_NAME: &str = "Docker Compose";
const MODULE_VERSION: &'static str = env!("CARGO_PKG_VERSION");

use anyhow::{Context, Result};
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use std::boxed::Box;
use std::collections::HashMap;

use super::schema::*;
use super::FoundryError;
use super::{AppInstance, AppTrait, ContainerTrait};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerCompose {
  /// A place to store find_app results
  /// Too many applications exist to enumerate them all, so we want to remember as many as possible
  // HACK: This should be a registry/cache rather than a simple hashmap
  app_cache: HashMap<String, AppInstance>,
  instance: AppInstance,
  config: Option<Schema>,
}

impl AppTrait for DockerCompose {
  fn get_name(&self) -> String {
    match &self.instance.version {
      Some(ver) => format!("{} ({})", APP_NAME, ver),
      None => "Bash (Unknown Version)".to_string(),
    }
  }

  fn build(instance: AppInstance) -> Result<DockerCompose> {
    Ok(DockerCompose {
      app_cache: HashMap::new(),
      instance: AppInstance {
        module_version: Some(DockerCompose::get_module_version()?),
        ..instance.clone()
      },
      config: None,
    })
  }

  /// Knows how to get the version number of the instaAppTrait
  /// Figures out how to call the cli using the given container
  fn set_cli(instance: AppInstance, container: Box<dyn ContainerTrait>) -> Result<AppInstance> {
    unimplemented!()
  }

  /// Knows how to get the version number of the installed app (not the module version)
  fn set_version(instance: AppInstance) -> Result<AppInstance> {
    unimplemented!()
  }
}

impl DockerCompose {
  fn get_module_version() -> Result<semver::Version> {
    Ok({
      semver::Version::parse(MODULE_VERSION).context(format!(
        "{} has an invalid version number '{}' Cargo.toml",
        APP_NAME, MODULE_VERSION
      ))
    }?)
  }

  pub fn load(&self, config_file: String) -> Result<DockerCompose> {
    log::debug!("reading the docker compose schema");

    log::info!("Parsing the docker compose file at {}", config_file);
    let contents = std::fs::read_to_string(config_file.clone()).context(format!(
      "Failed to open docker-compose file at {}",
      config_file
    ))?;
    let config = serde_yaml::from_str(&contents).context(format!(
      "Failed to parse the docker-compose file at {}",
      config_file
    ))?;

    log::debug!("Successfully parsed the schema at {}", config_file,);
    //     // let config = dc::File::read_from_path(std::path::Path::new(config_file));

    Ok(DockerCompose {
      config: Some(config),
      ..self.clone()
    })
  }

  //   pub fn act(&self, action: Action) -> Result<ActionResponse, Error> {
  //     action.run(self)
  //   }
}
// impl super::BackupTrait for DockerCompose {
//   fn run_init(conf: super::BackupConfig) -> Result<(), Error> {
//     log::debug!("Checking for path");
//     log::debug!("Checking volumes");
//     log::debug!("");
//     log::debug!("");
//     log::debug!("");
//     unimplemented!("No Run Init");
//   }
//   fn run_backup(conf: super::BackupConfig) -> Result<(), Error> {
//     unimplemented!("No Run Backup");
//   }
//   fn run_cleanup(conf: super::BackupConfig) -> Result<(), Error> {
//     unimplemented!("No Run Cleanup");
//   }
// }

// impl ApplicationTrait for DockerCompose {
//   fn get_schema(&self) -> Result<String, Error> {
//     unimplemented!("No schema for docker compose yet");
//   }

//   fn find_exe(&self, shell: Box<dyn ContainerTrait>) -> Result<String, Error> {
//     // NOTE: This seems that it should pull out to a Sh module
//   }

//   fn get_version(&self, exe: String) -> Result<String, Error> {
//     log::debug!("exe:\n{:#?}", exe);
//     let result = Command::new(exe).args(&["-v"]).output();
//     match result {
//       Ok(output) => Ok(
//         String::from_utf8(output.stdout)?
//           .trim_start_matches("docker-compose version")
//           .trim()
//           .to_string(),
//       ),
//       Err(err) => {
//         log::warn!("Could not find local executable for {}", err);
//         Err(Error::ApplicationError(format!("{}", err)))
//       }
//     }
//   }
// }

// Let examine messages for the foundry for communicating rather than directly returning values
#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub enum Event {
  UpComplete,
  DownComplete,
  RestartedService,
  BuildComplete,
}

/// The responses from the various actions.
///
/// It's easier to bundle them all up in an enum for schema purposes
#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub enum ActionResponse {
  ListServices(Vec<String>),
}

/// The actions exposed via API
#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub enum Action {
  /// Dump the configuration to the given file location. Useful for adding volumes/ports on the fly
  Export,
  /// List the services for a given config
  ListServices,
}

impl Action {
  fn run(&self, compose: &DockerCompose) -> Result<ActionResponse> {
    // We shouldn't be able to run anything without a valid configuration
    let conf = match &compose.config {
      Some(conf) => conf,
      None => Err(FoundryError::ConfigurationError).context(format!(
        "Docker Compose tried to run action {:#?} without a valid config",
        self
      ))?,
    };

    match self {
      &Action::Export => unimplemented!("Next up, Export"),
      &Action::ListServices => {
        log::debug!("Got List Services:");
        Ok(ActionResponse::ListServices(conf.list_service_names()))
      }
    }
  }
}

pub enum CliActions {
  /* -------    Cli Actions (To be pruned to only items to be exposed)   --------*/
  /// Build or rebuild services
  Build,
  ///Validate and view the Compose file
  Config,
  /// Create services
  Create,
  /// Stop and remove containers, networks, images, and volumes
  Down,
  /// Receive real time events from containers
  Events,
  /// Execute a command in a running container
  Exec,
  /// Get help on a command
  Help,
  /// List images
  Images,
  /// Kill containers
  Kill,
  /// View output from containers
  Logs,
  /// Pause services
  Pause,
  /// Print the public port for a port binding
  Port,
  /// List containers
  Ps,
  /// Pull service images
  Pull,
  /// Push service images
  Push,
  /// Restart services
  Restart,
  /// Remove stopped containers
  Rm,
  /// Run a one-off command
  Run,
  /// Set number of containers for a service
  Scale,
  /// Start services
  Start,
  /// Stop services
  Stop,
  /// Display the running processes
  Top,
  /// Unpause services
  Unpause,
  /// Create and start containers
  Up,
  /// Show the Docker-Compose version information
  Version,
}
