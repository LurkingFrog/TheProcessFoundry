//! Use and manage Bash shell
//!
//! This is essentially my hello world application and using it as a stalking horse for finding user stories

const APP_NAME: &str = "Bash";
const MODULE_VERSION: &'static str = env!("CARGO_PKG_VERSION");

use anyhow::{Context, Result};
use std::process::Command;

// This should likely be enumerated as it will be forked off into a separate project sooner rather than later
use super::*;

use serde_derive::{Deserialize, Serialize};

pub struct Bash {
  // The "As App" values
  pub path: Option<String>,
  pub version: Option<semver::Version>,
  // The "As container" values
  // pub apps: Registry?
}

impl AppTrait for Bash {
  fn get_name(&self) -> String {
    match &self.version {
      Some(ver) => format!("{} ({})", APP_NAME, ver),
      None => "Bash (Unknown Version)".to_string(),
    }
  }
}

pub struct BashFactory {}

impl BashFactory {
  pub fn new() -> BashFactory {
    BashFactory {}
  }
}

impl FactoryTrait for BashFactory {
  fn get_definition(&self) -> Result<AppDefinition> {
    let version = {
      semver::Version::parse(MODULE_VERSION).context(format!(
        "{} has an invalid version number '{}' Cargo.toml",
        APP_NAME, MODULE_VERSION
      ))
    }?;
    Ok(AppDefinition::new(APP_NAME.to_string(), version))
  }

  fn build(&self, container: Option<Box<dyn AppTrait>>) -> Result<Box<dyn AppTrait>> {
    // No container assumes it is the "local" copy we want. Some requires to just find the path/version otherwise
    // we need to ask the container (like docker)
    // it lives in. I'm doing this more for the info than from any need since I'm generally assuming
    // bash exists in each container.
    match container {
      Some(_contain) => unimplemented!("Finding the Remote Bash shell is not ready yet"),
      None => {
        let shell = Bash {
          path: Some("bash".to_string()),
          version: None,
        };
        let act = FindApp { search_paths: None };
        let def = AppDefinition {
          name: "bash".to_string(),
          ..Default::default()
        };

        let instance = match act.run(Box::new(shell), def)? {
          ActionResult::FindAppResult(result) => result,
          x => Err(FoundryError::Unreachable).context(format!(
            "Ran Bash::FindApp but did not get a FindAppResult:{:#?}",
            x
          ))?,
        };

        Ok(Box::new(Bash {
          path: instance.path,
          version: None,
        }))
      }
    }
  }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct FindApp {
  pub search_paths: Option<Vec<String>>,
  // pub case_insensitive: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
  Run(RunOptions),
  FindApp(FindApp),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionResult {
  // Run(RunResult),
  FindAppResult(AppInstance),
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct RunOptions {}

impl ActionTrait for FindApp {
  type RESPONSE = ActionResult;
  /// Find the first app that matches the conditions of AppDefinition (name, version,  path, etc)
  fn run(
    &self,
    container: Box<dyn AppTrait>,
    application: AppDefinition,
  ) -> Result<Self::RESPONSE> {
    let result = Command::new("bash")
      .args(&["-c", &format!("command -v {}", application.name)])
      .output();

    // THis should be another command based on ActionDefinition
    let version = "1.0.0";
    match result {
      Ok(output) => {
        let app = AppInstance {
          process_id: None,
          name: application.name,
          path: Some(String::from_utf8(output.stdout)?.trim().to_string()),
          container_id: None,
          factory_id: None,
          version: semver::Version::parse(version)?,
        };
        Ok(ActionResult::FindAppResult(app))
      }
      Err(err) => {
        let msg = format!(
          "{} could not find local executable for {}",
          container.get_name(),
          application.name
        );
        log::warn!("{}", msg);
        Err(FoundryError::NotFound).context(msg)
      }
    }
  }

  fn to_string(&self, application: Box<dyn AppTrait>) -> Result<Vec<String>> {
    unimplemented!("ActionTrait not implemented for shell")
  }
}
