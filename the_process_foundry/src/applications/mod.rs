//! A placeholder to group all the developmental applications

use super::FoundryError;
use super::{ActionTrait, AppTrait, ContainerTrait, LocalTrait};
use super::{AppInstance, AppQuery};

pub mod bash;
// pub mod docker;
pub mod docker_compose;
pub mod docker_container;
pub mod postgres;

pub use std::rc::Rc;

pub use bash::Bash;
// pub use docker::Docker;
pub use docker_compose::DockerCompose;
pub use docker_container::DockerContainer;
pub use postgres::Postgres;
