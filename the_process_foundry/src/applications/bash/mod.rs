//! Use and manage Bash shell
//!
//! This is essentially my hello world application and using it as a stalking horse for finding user stories

const APP_NAME: &str = "Bash";
const MODULE_VERSION: &'static str = env!("CARGO_PKG_VERSION");

use anyhow::{Context, Result};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;

// This should likely be enumerated as it will be forked off into a separate project sooner rather than later
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bash {
  /// A place to store find_app results
  /// Too many applications exist to enumerate them all, so we want to remember as many as possible
  // HACK: This should be a registry/cache rather than a simple hashmap
  app_cache: HashMap<String, AppInstance>,
  instance: AppInstance,
}

impl Bash {
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

impl LocalTrait for Bash {
  //
  fn get_local() -> Result<AppInstance> {
    // TODO: Actually fill out the app instance
    Ok(AppInstance::new("bash".to_string()))
  }
}

impl AppTrait for Bash {
  fn get_name(&self) -> String {
    self.get_name()
  }

  fn build(instance: AppInstance, _parent: Option<Rc<dyn ContainerTrait>>) -> Result<Bash> {
    Ok(Bash {
      app_cache: HashMap::new(),
      instance: AppInstance {
        module_version: Some(Bash::get_module_version()?),
        ..instance.clone()
      },
    })
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

impl ContainerTrait for Bash {
  /// This will find a list of apps with configurations that the container knows about
  fn find(&self, query: AppQuery) -> Result<Vec<AppInstance>> {
    match Action::FindApp(query).run(self.clone())? {
      ActionResult::FindAppResult(result) => Ok(result),
    }
  }

  /// List the known items in the app cache
  fn cached_apps(&self) -> Result<Vec<AppInstance>> {
    unimplemented!("No App Cache for Bash Yet")
  }

  fn forward(&self, to: AppInstance, message: Vec<String>) -> Result<String> {
    unimplemented!("No ContainerTrait::forward yet")
  }

  /// Get the name/version of the container, usually for use in logging/errors.
  fn get_name(&self) -> String {
    self.get_name()
  }
}

type FindApp = AppQuery;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
  Run(RunOptions),
  FindApp(FindApp),
}

// TODO: Make a derive for this
impl Action {
  fn run(&self, target: Bash) -> Result<ActionResult> {
    match self {
      Action::Run(_opts) => unimplemented!("Don't know how to run a command on Bash yet"),
      Action::FindApp(query) => query.run(target.instance),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionResult {
  // Run(RunResult),
  FindAppResult(Vec<AppInstance>),
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct RunOptions {}

impl ActionTrait for FindApp {
  type RESPONSE = ActionResult;
  /// Find the first app that matches the conditions of AppDefinition (name, version,  path, etc)
  fn run(&self, target: AppInstance) -> Result<Self::RESPONSE> {
    let result = Command::new("bash")
      .args(&["-c", &format!("command -v {}", self.name)])
      .output();

    // THis should be another command based on ActionDefinition
    match result {
      Ok(_output) => {
        // TODO: Set the networking/cli in the AppInstance
        let app = AppInstance::new(self.name.clone());
        Ok(ActionResult::FindAppResult(vec![app]))
      }
      Err(_err) => {
        let msg = format!(
          "{} could not find local executable for {}",
          target.full_name(),
          self.name,
        );
        log::warn!("{}", msg);
        Err(FoundryError::NotFound).context(msg)
      }
    }
  }

  fn to_string(&self, _target: AppInstance) -> Result<Vec<String>> {
    unimplemented!("ActionTrait not implemented for shell")
  }
}
