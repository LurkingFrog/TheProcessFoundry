//! The core definitions and structs for the Process Foundry
//!
//! After getting lost in the weeds, this is all the things that I seem to see come up as useful in every
//! application and conatainer
//!
//! # THINK: Additional traits
//!   - Actionable: Possibly part of app trait, since all should be able to utilize and emit Actions/Events
//!   - ActionResult: To abstract the result so we can pass it to something that has the code to actually use
//!   - Routable (possibly part of action)
//!   - consider https://abronan.com/rust-trait-objects-box-and-rc/ - Arc<Mutex<impl trait>>

use anyhow::{Context, Result};
use serde_derive::{Deserialize, Serialize};
use std::rc::Rc;

use super::Bash;
use super::FoundryError;

/// All applications should be introspective and able to build a running manager of itself
pub trait AppTrait {
  /// Construct the metadata for the module controlling the app instance
  /// This is important to make sure the module aligns with the actual installed instance
  fn build(instance: AppInstance, parent: Option<Rc<dyn ContainerTrait>>) -> Result<Self>
  where
    Self: Sized;

  /// Print a standardized "name (version)" for logging purposes
  fn get_name(&self) -> String;

  /// Knows how to get the version number of the installed app (not the module version)
  fn set_version(&self, instance: AppInstance) -> Result<AppInstance>;

  /// Figures out how to call the cli using the given container
  /// THINK: is it better to have an option or make "Local" a special case?
  fn set_cli(
    &self,
    instance: AppInstance,
    container: Rc<dyn ContainerTrait>,
  ) -> Result<AppInstance>;
}

/// Ways to manage applications (eg Docker, Bash) contained within itself
pub trait ContainerTrait: std::fmt::Debug {
  /// This will find a list of apps with configurations that the container knows about
  fn find(&self, query: AppQuery) -> Result<Vec<AppInstance>>;

  /// Find a unique app that matches the query
  fn find_one(&self, query: AppQuery) -> Result<AppInstance> {
    let all = self.find(query.clone())?;
    match all.len() {
      0 => Err(FoundryError::NotFound).context(format!(
          "No instances matching your query of {} have been registered",
          query.name
      )),
      1 => Ok(all.get(0).unwrap().clone()),
      x => Err(FoundryError::MultipleMatches).context(format!(
          "{} instances matching your defition for {} have been registered. Please narrow your search criteria",
          x, query.name
      )),
    }
  }

  /// Send a stringified action to the AppInstance
  fn forward(&self, to: AppInstance, message: Message) -> Result<String>;

  /// List the known items in the app cache
  fn cached_apps(&self) -> Result<Vec<AppInstance>>;

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

  // Searches high and low for apps that match the criteria. If false it returns an error with multiple matches.
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
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AppInstance {
  /// This is for use by Containers enumerating instances of their content
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShellType {
  Dash,
  Bash,
  Zsh,
}

/// A special case for bootstrapping. I'm trying to find the enumerations that actually deserve to be
/// traits themselves
#[derive(Debug, Clone)]
pub struct Shell {
  pub shell_type: ShellType,
  pub instance: AppInstance,
  pub running: Rc<dyn ContainerTrait>,
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

    let (instance, running) = match shell_type {
      ShellType::Bash => {
        let instance = Bash::get_local().context("Could not get a local Bash shell")?;
        (instance.clone(), Rc::new(Bash::build(instance, None)?))
      }
      _ => unreachable!("Should currently not be able to use any local shell other than bash"),
    };

    Ok(Shell {
      instance,
      shell_type,
      running,
    })
  }
}

// THINK: This is very specific to forwarding to shell and is more like a script. Does this belong with
//        the future workflows?
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cmd {
  pub run_as: Option<String>,
  pub command: String,
  pub args: Vec<String>,
}

///  A generic message designed to be sent to a container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
  /// For use in a remote shell, like one contained within a docker container
  Command(Cmd),

  /// Build an Rpc call
  Rpc,

  /// Call a Restful API
  Rest,
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
