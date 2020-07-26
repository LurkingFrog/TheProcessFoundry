//! A generic Shell wrapper
//!
//! This is used for interfacing with its command line interface (CLI).
//!
//! In the future, it can also be used to generate scripts in a given language and saved to an outside file,
//! so they can be stored, modified, and run directly by users without need of the TPF.
//!
//!

use anyhow::{Context, Result};
use serde_derive::{Deserialize, Serialize};
use std::rc::Rc;

use super::{AppInstance, AppQuery, AppTrait, ContainerTrait, Message};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShellType {
  Dash,
  Bash,
  Zsh,
}

#[derive(Debug, Clone)]
pub struct Shell {
  pub shell_type: ShellType,
  pub instance: AppInstance,
}

impl Shell {
  /// Currently a special case, search the local filesystem directly
  /// HACK: This needs to be converted to the generic find function
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
        (instance.clone(), Rc::new(Bash::new(instance, None)?))
      }
      _ => unreachable!("Should currently not be able to use any local shell other than bash"),
    };

    Ok(Shell {
      instance,
      shell_type,
    })
  }
}

impl ContainerTrait for Shell {
  /// This will find a list of apps with configurations that the container knows about
  fn find(&self, query: AppQuery) -> Result<Vec<AppInstance>> {
    unimplemented!("ContainerTrait::Shell::")
  }

  /// Send a stringified action to the AppInstance
  fn forward(&self, to: AppInstance, message: Message) -> Result<String> {
    unimplemented!("ContainerTrait::Shell::")
  }

  // fn deliver(app: Rc<dyn AppTrait>) -> Result<Ack?> {}

  /// List the known items in the app cache
  fn cached_apps(&self) -> Result<Vec<AppInstance>> {
    unimplemented!("ContainerTrait::Shell::")
  }

  /// Get the name/version of the container, usually for use in logging/errors.
  fn get_name(&self) -> String {
    unimplemented!("ContainerTrait::Shell::")
  }
}
