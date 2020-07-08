//! Information about a running container
//!
//! This is going to use shiplift since they seem to have done most of the work already.
//! This is independent of docker because there are multiple ways of manipulating these (docker,
//! docker-compose, etc.) so we're going to make a specific container to hold the metadata.

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
  Up,
  Exited,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerContainer {
  status: Status,
  instance: AppInstance,
}

impl DockerContainer {
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

  fn build(instance: AppInstance) -> Result<DockerContainer> {
    Ok(DockerContainer {
      status: Status::Down,
      instance: AppInstance {
        module_version: Some(DockerContainer::get_module_version()?),
        ..instance.clone()
      },
    })
  }

  /// Knows how to get the version number of the installed app (not the module version)
  fn set_version(_instance: AppInstance) -> Result<AppInstance> {
    unimplemented!()
  }
  /// Figures out how to call the cli using the given container
  fn set_cli(_instance: AppInstance, _container: Box<dyn ContainerTrait>) -> Result<AppInstance> {
    unimplemented!()
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
  Inspect(InspectOptions),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionResult {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InspectOptions {}

impl ActionTrait for InspectOptions {
  type RESPONSE = ActionResult;

  fn run(&self, _target: AppInstance) -> Result<Self::RESPONSE> {
    unimplemented!()
  }

  fn to_string(&self, _target: AppInstance) -> Result<Vec<String>> {
    unimplemented!("ActionTrait not implemented for shell")
  }
}
