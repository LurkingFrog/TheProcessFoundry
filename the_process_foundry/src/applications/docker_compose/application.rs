//! Docker-Compose functionality
//!
//! A separate app to examine and run docker compose

const APP_NAME: &str = "Docker Compose";
const MODULE_VERSION: &'static str = env!("CARGO_PKG_VERSION");

use anyhow::{Context, Result};
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;

use super::schema::*;
use super::FoundryError;
use super::{docker_container, DockerContainer};
use super::{ActionTrait, AppInstance, AppQuery, AppTrait, Cmd, ContainerTrait, Message};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerCompose {
  /// We want to put the shell/parent container here
  #[serde(skip)]
  parent: Option<Rc<dyn ContainerTrait>>,
  // docker: Docker,
  instance: AppInstance,
  config: Option<Schema>,
  containers: HashMap<String, DockerContainer>,
}

impl AppTrait for DockerCompose {
  fn get_name(&self) -> String {
    match &self.instance.version {
      Some(ver) => format!("{} ({})", APP_NAME, ver),
      None => format!("{} (Unknown Version)", APP_NAME),
    }
  }

  fn build(instance: AppInstance, parent: Option<Rc<dyn ContainerTrait>>) -> Result<DockerCompose> {
    Ok(DockerCompose {
      parent,
      instance: AppInstance {
        module_version: Some(DockerCompose::get_module_version()?),
        ..instance.clone()
      },
      config: None,
      containers: Default::default(),
    })
  }

  /// Knows how to get the version number of the instaAppTrait
  /// Figures out how to call the cli using the given container
  fn set_cli(
    &self,
    _instance: AppInstance,
    _container: Rc<dyn ContainerTrait>,
  ) -> Result<AppInstance> {
    unimplemented!()
  }

  /// Knows how to get the version number of the installed app (not the module version)
  fn set_version(&self, _instance: AppInstance) -> Result<AppInstance> {
    unimplemented!()
  }
}

impl ContainerTrait for DockerCompose {
  /// This will find a list of apps with configurations that the container knows about
  fn find(&self, query: AppQuery) -> Result<Vec<AppInstance>> {
    let conf = self.get_conf()?;
    Ok(
      conf
        .list_service_names()
        .into_iter()
        .filter_map(|item| match item.to_lowercase() == query.name {
          false => None,
          true => Some(AppInstance::new(item)),
        })
        .collect(),
    )
  }

  /// List the known items in the app cache
  fn cached_apps(&self) -> Result<Vec<AppInstance>> {
    unimplemented!("No App Cache for Bash Yet")
  }

  /// Send the message to a child item
  fn forward(&self, to: AppInstance, message: Message) -> Result<String> {
    match message {
      Message::Command(cmd) => {
        let exec = ExecOptions {
          service_name: to.name,
          user: cmd.run_as.clone(),
          command: cmd.command,
          args: cmd.args,
          ..Default::default()
        };
        match exec.run(self.instance.clone())? {
          ActionResult::Exec(val) => Ok(val),
          err => Err(FoundryError::UnexpectedValue).context(format!(
            "Running DockerCompose::ExecOptions did not return an ExecResult:\n{:#?}",
            err
          )),
        }
      }
      _ => Err(FoundryError::UnexpectedValue)
        .context("Docker Compose tried to forward a non-command to a container"),
    }
  }

  /// Get the name/version of the container, usually for use in logging/errors.
  fn get_name(&self) -> String {
    self.get_name()
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

  fn get_name(&self) -> String {
    match &self.instance.version {
      Some(ver) => format!("{} ({})", APP_NAME, ver),
      None => format!("{} (Unknown Version)", APP_NAME),
    }
  }

  fn get_conf(&self) -> Result<Schema> {
    match self.config.clone() {
      Some(conf) => Ok(conf),
      None => Err(FoundryError::ConfigurationError).context(
        "Docker Compose does not have a loaded configuration. Make sure DockerCompose::load is used first"
      )?,
    }
  }

  /// Load the configuration from an existing yaml file
  pub fn load(&self, config_file: String) -> Result<DockerCompose> {
    log::debug!("reading the docker compose schema");

    log::info!("Parsing the docker compose file at {}", config_file);
    let contents = std::fs::read_to_string(config_file.clone()).context(format!(
      "Failed to open docker-compose file at {}",
      config_file
    ))?;

    let conf = Schema {
      source: Some(config_file.clone()),
      ..serde_yaml::from_str(&contents).context(format!(
        "Failed to parse the docker-compose file at {}",
        config_file
      ))?
    };

    log::debug!("Successfully parsed the schema at {}", config_file,);

    log::debug!("Getting the docker containers");
    let containers = HashMap::new();
    let mut new_compose = DockerCompose {
      config: Some(conf.clone()),
      containers,
      instance: AppInstance {
        config_file: Some(config_file.clone()),
        ..self.instance.clone()
      },
      ..self.clone()
    };

    for name in conf.list_service_names() {
      new_compose
        .containers
        .insert(name.clone(), new_compose.define_container(name)?);
    }

    log::debug!("Returning the compose");
    Ok(new_compose)
  }

  /// Private function used to build a container from the schema
  /// TODO: Add in result from "docker inspect" if running
  /// TODO: If status is "Up", we want to get/set shell
  fn define_container(&self, name: String) -> Result<DockerContainer> {
    let instance = AppInstance::new(name.clone());
    DockerContainer::build(instance, Some(Rc::new(self.clone())))
  }

  /// Set the status of all the services that this instance knows about
  fn _update_status(&mut self, _name: String) -> Result<()> {
    Ok(())
  }

  pub fn run_action(&self, action: Action) -> Result<ActionResult> {
    action.run(self.clone())
  }

  pub fn get_container(&self, name: String) -> Result<DockerContainer> {
    match self.containers.get(&name) {
      Some(container) => Ok(container.clone()),
      None => {
        let conf = self
          .get_conf()
          .context("Failed to run DockerCompose::get_container")?;
        Err(FoundryError::NotFound).context(format!(
          "Docker Compose does not have a service named '{}' in conf at '{}'. Possible choices are: {:#?} ",
          name,
          conf.get_source(),
          conf.list_service_names(),
        ))
      }
    }
  }

  // Cli functions will go here
}

// Let examine messages for the foundry for communicating rather than directly returning values
#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub enum Event {
  UpComplete,
  DownComplete,
  RestartedService,
  BuildComplete,
}

/// The actions registered to the system.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Action {
  Find(FindApp),
  // Run the exec command against a running container
  Exec(ExecOptions),
  /// Dump the configuration to the given file location. Useful for adding volumes/ports on the fly
  Export,
}

/// The responses from the various actions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionResult {
  // Run(RunResult),
  FindResult(Vec<AppInstance>),
  ListServices(Vec<String>),
  Exec(String),
}

impl Action {
  fn run(&self, compose: DockerCompose) -> Result<ActionResult> {
    match self {
      Action::Export => unimplemented!("Next up, Export"),
      Action::Find(query) => match query.0.find_all {
        true => Ok(ActionResult::FindResult(compose.find(query.0.clone())?)),
        false => Ok(ActionResult::FindResult(vec![
          compose.find_one(query.0.clone())?
        ])),
      },
      Action::Exec(opts) => opts.run(compose.instance.clone()),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindApp(AppQuery);

impl ActionTrait for FindApp {
  type RESPONSE = ActionResult;

  fn run(&self, _target: AppInstance) -> Result<Self::RESPONSE> {
    unimplemented!("Still haven't figured out Actions yet")
  }

  fn to_message(&self, target: Option<AppInstance>) -> Result<Vec<Message>> {
    unimplemented!("ActionTrait not implemented for shell")
  }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExecOptions {
  command: String,
  args: Vec<String>,
  service_name: String,
  detach: bool,
  privileged: bool,
  index: Option<u8>,
  user: Option<String>,
  env: Option<HashMap<String, String>>,
  workdir: Option<String>,
}

impl ExecOptions {
  pub fn new(service_name: String, command: String) -> ExecOptions {
    ExecOptions {
      service_name,
      command,
      ..Default::default()
    }
  }
}

// "/usr/local/bin/docker-compose" "-f" "/home/dfogelson/Foundry/TheProcessFoundry/the_process_foundry/tests/data/postgres.docker-compose.yml" "exec" "-T" "postgres" "bash" "-c" "command -v pg_basebackup"
fn command_str(command: &std::process::Command) -> Result<String> {
  let cmd = format!("{:#?}", command);
  let regex = regex::Regex::new(r#"(^\s*")|(" ")|("\s*$)"#)?;
  Ok(regex.replace_all(&cmd, " ").to_string())
}

impl ActionTrait for ExecOptions {
  type RESPONSE = ActionResult;

  fn run(&self, compose: AppInstance) -> Result<Self::RESPONSE> {
    // TODO: change this to use the instance path
    let mut cmd = std::process::Command::new(format!("/usr/local/bin/{}", compose.name));

    // Add the option args
    match compose.config_file {
      Some(path) => cmd.arg("-f").arg(path.clone()),
      // TODO: Actually make the write function exist
      None => Err(FoundryError::ConfigurationError).context(
        "Docker Compose does not configuration file. TODO: Use DockerCompose::write to create one",
      )?,
    };
    cmd.arg("exec").arg("-T");
    match &self.user {
      None => (),
      Some(user) => {
        cmd.arg("--user");
        cmd.arg(&user.clone());
      }
    };
    cmd.arg(self.service_name.clone());

    // And add the command
    cmd.arg(self.command.clone());
    cmd.args(&self.args);
    log::debug!("Docker compose is executing a cmd:\n{}", command_str(&cmd)?);
    let result = cmd.arg("").output()?;
    match result.status.success() {
      true => Ok(ActionResult::Exec(
        String::from_utf8(result.stdout)?.trim_end().to_string(),
      )),
      false => {
        Err(FoundryError::RemoteError).context("Received an error trying to run in docker compose")
      }
    }
  }

  fn to_message(&self, target: Option<AppInstance>) -> Result<Vec<Message>> {
    unimplemented!("ActionTrait not implemented for shell")
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
