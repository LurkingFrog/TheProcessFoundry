//! A message to change the state of the components that receive it
//!
//! The only item this should return is an optional "ack" to the requestor, compiled from all the the
//! components which received the request. All results will be returned either through the event interface
//! or a callback URL. This allows us to have

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize)]
pub struct ActionOptions {}

impl std::fmt::Display for ActionOptions {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:#?}", self)
  }
}

impl ActionOptions {}
