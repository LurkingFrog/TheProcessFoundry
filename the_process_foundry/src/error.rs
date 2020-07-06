//! Errors that can be generated by the Process Foundry
//!
//! We rewrap all the modules errors so the end coder can catch a fully enumerated list of errors

use serde_derive::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Error, Deserialize, Serialize)]
pub enum FoundryError {
  #[error("There was an error attempting to convert from one type to another")]
  ConversionError,

  #[error("The application has a problem with the configuration")]
  ConfigurationError,

  #[error("This item's key is already in use")]
  DuplicateKeyError,

  #[error("Received multiple instances when looking for a single unique response")]
  MultipleMatches,

  #[error("The item you were looking for was not found")]
  NotFound,

  #[error("We received an error that was not explicitly handled")]
  UnhandledError,

  #[error("Got to a surprise option in a match statement")]
  Unreachable,
}

impl From<std::num::ParseIntError> for FoundryError {
  fn from(err: std::num::ParseIntError) -> FoundryError {
    log::warn!("Received Parse Int Error:\n{:#?}", err);
    FoundryError::ConversionError
  }
}

impl From<std::num::ParseFloatError> for FoundryError {
  fn from(err: std::num::ParseFloatError) -> FoundryError {
    log::warn!("Received Parse Float Error:\n{:#?}", err);
    FoundryError::ConversionError
  }
}

impl From<std::string::FromUtf8Error> for FoundryError {
  fn from(err: std::string::FromUtf8Error) -> FoundryError {
    log::warn!(
      "Received Error converting from utf8 into a String:\n{:#?}",
      err
    );
    FoundryError::ConversionError
  }
}

// impl From<std::io::Error> for FoundryError {
//   fn from(err: std::io::Error) -> FoundryError {
//     FoundryError::new(ErrorKind::IOError, err.to_string())
//       .add_original_type("std::io::Error".to_string())
//   }
// }

// impl From<semver::ReqParseError> for FoundryError {
//   fn from(err: semver::ReqParseError) -> FoundryError {
//     FoundryError::new(ErrorKind::ParsingError, err.to_string())
//       .add_original_type("semver::ReqParseError".to_string())
//   }
// }

// impl From<semver::SemVerError> for FoundryError {
//   fn from(err: semver::SemVerError) -> FoundryError {
//     FoundryError::new(ErrorKind::ParsingError, err.to_string())
//       .add_original_type("semver::SemVerError".to_string())
//   }
// }
