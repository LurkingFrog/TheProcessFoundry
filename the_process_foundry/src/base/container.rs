//! An object for holding applications and and other containers
//!
//! A container is primarily a node in a routing tree. It has two functions, knowing how to lazily find out
//! its own children and passing messages.
//!
//! The container interface is designed to imitate how network routing works, both broadcast (everybody who
//! cares) and direct (specific instance). The goal is to have a distributed pub/sub messaging queue that can
//! minimize network traffic yet make sure everybody who cares about a particular message will eventually get
//! it.
//!

use anyhow::{Context, Result};
use std::rc::Rc;

use super::FoundryError;
use super::{AppInstance, AppQuery, AppTrait, Message};

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
          "{} instances matching your definition for {} have been registered. Please narrow your search criteria",
          x, query.name
      )),
    }
  }

  /// Send a stringified action to the AppInstance
  fn forward(&self, to: AppInstance, message: Message) -> Result<String>;

  // fn deliver()

  /// List the known items in the app cache
  fn cached_apps(&self) -> Result<Vec<AppInstance>>;

  /// Get the name/version of the container, usually for use in logging/errors.
  fn get_name(&self) -> String;
}

#[derive(Clone, Debug)]
pub enum CacheItem {
  App(Rc<dyn AppTrait>),
  Container(Rc<dyn ContainerTrait>),
}

impl std::fmt::Display for CacheItem {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:#?}", self)
  }
}

/// TODO: Rename this
/// A message forwarding system that remembers where items have gone before
#[derive(Clone, Debug, Default)]
pub struct ContainerRouter {
  parent: Option<Rc<Container>>,
  cache: std::collections::HashMap<String, CacheItem>,
}

impl std::fmt::Display for ContainerRouter {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:#?}", self)
  }
}

impl ContainerRouter {}

// ******************************************************************************************************  //

/// An example implementation of a container.
#[derive(Clone, Debug)]
pub struct Container {
  relatives: ContainerRouter,
  interface: Rc<dyn ContainerTrait>,
  app: Option<Rc<dyn AppTrait>>,
}

impl std::fmt::Display for Container {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:#?}", self)
  }
}

impl Container {}
