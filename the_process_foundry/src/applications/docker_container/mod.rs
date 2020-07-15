//! Information about a running container
//!
//! This is going to use shiplift since they seem to have done most of the work already.
//! This is independent of docker because there are multiple ways of manipulating these (docker,
//! docker-compose, etc.) so we're going to make a specific container to hold the metadata.
//! THINK: Should this assume it is clean (freshly spun up) or can it be dirty?

const APP_NAME: &str = "Docker Container";
const MODULE_VERSION: &'static str = env!("CARGO_PKG_VERSION");

use anyhow::{Context, Result};
use serde_derive::{Deserialize, Serialize};
// use schemars::JsonSchema;
// use shiplift::Docker;

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Status {
  Down,
  /// Have not looked up the status
  Unknown,
  /// The container is currently running
  Up,
  /// The container is being reported as done
  Exited,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerContainer {
  pub status: Status,
  pub instance: AppInstance,

  /// The container who owns this instance, and how we send manipulation commands (eg Docker, DockerCompose)
  #[serde(skip)]
  pub parent: Option<Rc<dyn ContainerTrait>>,

  /// The shell to use inside this container when running additional executables
  #[serde(skip)]
  pub shell: Option<Rc<dyn ContainerTrait>>,
}

impl DockerContainer {
  /// If we don't have a parent, set it to a local instance of docker
  /// TODO: Actually make this happen. Currently already using Docker-Compose
  pub fn set_parent(&self, parent: Rc<dyn ContainerTrait>) -> Result<DockerContainer> {
    match &self.parent {
      Some(x) => log::info!(
        "Replacing parent {} on container {} with {}",
        x.get_name(),
        self.get_name(),
        parent.get_name()
      ),
      None => (),
    }
    Ok(DockerContainer {
      parent: Some(parent),
      ..self.clone()
    })
  }

  /// Find and verify the shell on the container
  pub fn set_shell(&self, preferred: Option<AppQuery>) -> Result<DockerContainer> {
    let query = preferred.unwrap_or(AppQuery::new("bash".to_string()));

    match &self.shell {
      Some(x) => log::info!(
        "Replacing shell {} on container {} with {}",
        x.get_name(),
        self.get_name(),
        query.name
      ),
      None => (),
    }

    // HACK: Still trying to get my head around this special case
    let shell = Rc::new(Bash::build(AppInstance::new("bash".to_string()), None)?);

    Ok(DockerContainer {
      shell: Some(shell),
      ..self.clone()
    })
  }

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
}

impl AppTrait for DockerContainer {
  fn get_name(&self) -> String {
    self.get_name()
  }

  fn build(
    instance: AppInstance,
    parent: Option<Rc<dyn ContainerTrait>>,
  ) -> Result<DockerContainer> {
    let base = DockerContainer {
      status: Status::Down,
      instance: AppInstance {
        module_version: Some(DockerContainer::get_module_version()?),
        ..instance.clone()
      },
      parent,
      shell: None,
    };
    base.set_shell(None)
  }

  /// Knows how to get the version number of the installed app (not the module version)
  fn set_version(&self, _instance: AppInstance) -> Result<AppInstance> {
    unimplemented!()
  }
  /// Figures out how to call the cli using the given container
  fn set_cli(
    &self,
    _instance: AppInstance,
    _container: Rc<dyn ContainerTrait>,
  ) -> Result<AppInstance> {
    unimplemented!()
  }
}

impl ContainerTrait for DockerContainer {
  /// This will find a list of apps with configurations that the container knows about
  fn find(&self, query: AppQuery) -> Result<Vec<AppInstance>> {
    // Is there a shell
    let shell = match &self.shell {
      None => Err(FoundryError::NotConfigured).context(format!(
        "Cannot run find: No shell set in '{}'",
        self.get_name()
      ))?,
      Some(x) => x,
    };

    // Is there a parent container
    let parent = match &self.parent {
      None => Err(FoundryError::NotConfigured).context(format!(
        "Cannot run find: No parent set in '{}'",
        self.get_name()
      ))?,
      Some(x) => x,
    };

    // HACK: This should be generic container work, but not quite sure how to make it so

    let regex = regex::Regex::new(r"^(\w+) \(([\w\.\s]+)\)$")?;
    let lc = shell.get_name().to_lowercase();
    let name = regex.captures(&lc).map_or(
      Err(FoundryError::UnexpectedValue).context(format!(
        "'{}' does not appear to be a valid shell name",
        shell.get_name()
      )),
      |cap| {
        cap.get(1).map_or(
          Err(FoundryError::UnexpectedValue).context(format!(
            "'{}' does not appear to be a valid shell name",
            shell.get_name()
          )),
          |val| Ok(val.as_str()),
        )
      },
    )?;

    let cmd = match name {
      "bash" => Ok(Message::Command(Cmd {
        command: "bash".to_string(),
        args: ["-c", &format!("command -v {}", query.name)]
          .iter()
          .map(|x| x.to_string())
          .collect(),
      })),
      _ => Err(FoundryError::UnexpectedValue).context(format!(
        "Docker containers are not currently set to use '{}' shells",
        name
      )),
    }?;

    let result = parent.forward(self.instance.clone(), cmd)?;
    log::debug!("Forward result:\n{:#?}", result);
    unimplemented!()
  }

  /// List the known items in the app cache
  fn cached_apps(&self) -> Result<Vec<AppInstance>> {
    unimplemented!("No App Cache for Bash Yet")
  }

  fn forward(&self, to: AppInstance, message: Message) -> Result<String> {
    unimplemented!("No ContainerTrait::forward yet")
  }

  /// Get the name/version of the container, usually for use in logging/errors.
  fn get_name(&self) -> String {
    self.get_name()
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindApp(AppQuery);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
  Find(FindApp),
  Inspect(InspectOptions),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionResult {
  FindResult(Vec<AppInstance>),
  InspectResult,
}

impl Action {
  fn _run(&self, container: DockerContainer) -> Result<ActionResult> {
    // We shouldn't be able to run anything without a valid configuration
    // let conf = match &compose.config {
    //   Some(conf) => conf,
    //   None => Err(FoundryError::ConfigurationError).context(format!(
    //     "Docker Compose tried to run action {:#?} without a valid config",
    //     self
    //   ))?,
    // };

    // if status is not Run, start and flag

    // if container.parent is empty, assume there is a local docker to use

    // if container.shell is empty, attempt to look up bash in the container

    match self {
      Action::Inspect(_) => unimplemented!("Next up, Inspect"),
      Action::Find(_query) => unimplemented!(),
    }
  }
}

impl ActionTrait for FindApp {
  type RESPONSE = ActionResult;

  fn run(&self, _target: AppInstance) -> Result<Self::RESPONSE> {
    unimplemented!()
  }

  fn to_string(&self, _target: Option<AppInstance>) -> Result<String> {
    unimplemented!("ActionTrait not implemented for shell")
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InspectOptions {}

impl ActionTrait for InspectOptions {
  type RESPONSE = ActionResult;

  fn run(&self, _target: AppInstance) -> Result<Self::RESPONSE> {
    unimplemented!()
  }

  fn to_string(&self, _target: Option<AppInstance>) -> Result<String> {
    unimplemented!("ActionTrait not implemented for shell")
  }
}
