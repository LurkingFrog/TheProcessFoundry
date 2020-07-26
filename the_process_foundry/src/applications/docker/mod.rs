//! Directly control the Docker CLI
//!
//! THINK: Does a DOCKERFILE parser belong in here?

const APP_NAME: &str = "Docker";
const MODULE_VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Docker {
  parent: Option<Rc<dyn ContainerTrait>>,
  instance: AppInstance,
  config: Option<Schema>,
  containers: HashMap<String, DockerContainer>,
}

impl AppTrait for DockerCompose {
  fn get_name(&self) -> String {
    match &self.instance.version {
      Some(ver) => format!("{} ({})", APP_NAME, ver),
      None => format!("{} (Unknown Version)", APP_NAME),
    }
  }

  fn new(instance: AppInstance) -> Result<DockerCompose> {
    Ok(DockerCompose {
      instance: AppInstance {
        module_version: Some(DockerCompose::get_module_version()?),
        ..instance.clone()
      },
      config: None,
      containers: Default::default(),
    })
  }

  /// Knows how to get the version number of the instaAppTrait
  /// Figures out how to call the cli using the given container
  fn set_cli(_instance: AppInstance, _container: Rc<dyn ContainerTrait>) -> Result<AppInstance> {
    unimplemented!()
  }

  /// Knows how to get the version number of the installed app (not the module version)
  fn set_version(_instance: AppInstance) -> Result<AppInstance> {
    unimplemented!()
  }
}

impl ContainerTrait for Docker {
  /// This will find a list of apps with configurations that the container knows about
  fn find(&self, query: AppQuery) -> Result<Vec<AppInstance>> {
    unimplemented!()
  }

  /// List the known items in the app cache
  fn cached_apps(&self) -> Result<Vec<AppInstance>> {
    unimplemented!("No App Cache for Bash Yet")
  }

  /// Get the name/version of the container, usually for use in logging/errors.
  fn get_name(&self) -> String {
    self.get_name()
  }
}
