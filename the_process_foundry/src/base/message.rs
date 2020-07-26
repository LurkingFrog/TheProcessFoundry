//! An encapsulated set of data containing routing information
//!
//! Much like TCP/IP, each message can be inspected to figure out where it gets routed. Keeping this standard
//! means we can convert between transport media easily such as putting it on a message queue

use serde_derive::{Deserialize, Serialize};
use std::rc::Rc;

use super::FoundryError;

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
pub enum MessageValue {
  /// For use in a remote shell, like one contained within a docker container
  Command(Cmd),

  /// Build an Rpc call
  Rpc,

  /// Call a Restful API
  Rest,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize)]
pub struct Message {
  routing: RoutingInfo,
  header: String,
  body: String,
}

impl std::fmt::Display for Message {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:#?}", self)
  }
}

impl Message {}

/// Definition of where to send a message
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize)]
pub struct RoutingInfo {
  targets: Vec<RoutingTarget>,
}

impl std::fmt::Display for RoutingInfo {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:#?}", self)
  }
}

impl RoutingInfo {}

///
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum RoutingTarget {
  Broadcast(),
  Children,
}

impl std::fmt::Display for RoutingTarget {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:#?}", self)
  }
}

impl RoutingTarget {}
