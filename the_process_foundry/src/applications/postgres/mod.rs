//! Module for manipulating a Postgres server
//!
//! THINK: What is the scope of this module. Does it include managing the internal data?

const APP_NAME: &str = "Postgres";
const MODULE_VERSION: &'static str = env!("CARGO_PKG_VERSION");

use anyhow::{Context, Result};
use serde_derive::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Postgres {
  /// A place to store find_app results
  /// Too many applications exist to enumerate them all, so we want to remember as many as possible
  instance: AppInstance,
}

impl Postgres {
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

impl AppTrait for Postgres {
  fn get_name(&self) -> String {
    self.get_name()
  }

  fn new(instance: AppInstance, _parent: Option<Rc<dyn ContainerTrait>>) -> Result<Postgres> {
    Ok(Postgres {
      instance: AppInstance {
        module_version: Some(Postgres::get_module_version()?),
        ..instance.clone()
      },
    })
  }

  /// Knows how to get the version number of the installed app (not the module version)
  fn set_version(&self, _instance: AppInstance) -> Result<AppInstance> {
    unimplemented!()
  }
  // /// Figures out how to call the cli using the given container
  // fn set_cli(
  //   &self,
  //   _instance: AppInstance,
  //   _container: Rc<dyn ContainerTrait>,
  // ) -> Result<AppInstance> {
  //   unimplemented!()
  // }
}
