//! A placeholder to group all the developmental applications

use super::FoundryError;
use super::{ActionTrait, AppTrait, ContainerTrait, LocalTrait};
use super::{AppInstance, AppQuery};

pub mod bash;
pub mod docker_compose;

pub use bash::Bash;
pub use docker_compose::DockerCompose;
