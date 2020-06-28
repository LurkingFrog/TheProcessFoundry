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
  pub path: Option<String>,
  pub version: Option<semver::Version>,
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
  fn build(&self) -> Result<Box<dyn AppTrait>> {
    unimplemented!("")
  }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct FindAppOptions {
  pub search_paths: Option<Vec<String>>,
  // pub case_insensitive: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
  Run(RunOptions),
  FindApp(FindAppOptions),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionResult {
  // Run(RunResult),
  FindAppResult(Vec<Box<AppInstance>>),
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct RunOptions {}

impl ActionTrait for FindAppOptions {
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
        let apps = vec![Box::new(AppInstance {
          process_id: None,
          name: application.name,
          path: Some(String::from_utf8(output.stdout)?.trim().to_string()),
          container_id: None,
          factory_id: None,
          version: semver::Version::parse(version)?,
        })];
        Ok(ActionResult::FindAppResult(apps))
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
