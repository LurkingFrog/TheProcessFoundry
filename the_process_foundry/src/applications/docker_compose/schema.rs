//! Schema for a docker-compose configuration file
//!
//! This is the data structure for holding the yaml file that docker compose uses to do its thing.
//! I'm only doing the fields for features I'm using as a proof of concept. This schema structure is only
//! going to be explicitly used by the application, so it should all be private and accessed by
//! getters/setters.
//!
//! TODO: Write stand alone CLI tool to convert JSON to structs "schema.rs" or some such.
//! TODO: Write CLI tool to diff schema versions
//! TOOD: Figure out versioning (#[serde_semver(x)], where x is semver::VersionReq.
//!       https://docs.rs/semver/0.10.0/semver/struct.VersionReq.html)

use serde_derive::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A Schema structure to contain all of the possible values that can be contained in a docker-compose.yml
/// This is going to be incomplete, only adding things as I implement functions. See
/// https://github.com/docker/compose/tree/master/compose/config for the JSON definitions of this.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Schema {
  // TODO: Change this to a AppTrait, so we can give remote options like git, curl or file
  /// The location of the serialized copy of the schema
  #[serde(skip)]
  pub source: Option<String>,
  pub version: String,
  pub services: BTreeMap<String, Service>,
  // networks: Vec<Network>,
  // volumes: Vec<Volume>,
  // secrets: Vec<Secret>,
}

impl Default for Schema {
  fn default() -> Self {
    Self {
      source: None,
      version: "3.8".to_string(),
      services: Default::default(),
    }
  }
}

impl Schema {
  pub fn get_source(&self) -> String {
    match &self.source {
      Some(x) => x.clone(),
      None => "No source set".to_string(),
    }
  }

  pub fn list_service_names(&self) -> Vec<String> {
    self.services.keys().map(|key| key.clone()).collect()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Service {
  build: Option<Build>,
  image: Option<String>,
  command: Option<Vec<String>>,
  depends_on: Option<Vec<String>>,
  ports: Option<Vec<Port>>, //Unique
  restart: Option<Restart>,
  volumes: Option<Vec<ServiceVolume>>,
  // deploy: Option<Deployment>,
  // cap_add: Vec<String>, // Unique
  // cap_drop: Vec<String>, //Unique
  // cgroup_parent: Option<String>,
  // configs: Vec<Config>,
  // container_name: Option<String>,
  // credential_spec: Option<CredentialSpec>,
  // devices: Vec<String>, //Unique
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Build {}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Port {
  Str(String),
  Obj(PortObj),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PortObj {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Restart {
  #[serde(rename = "no")]
  No,
  #[serde(rename = "always")]
  Always,
  #[serde(rename = "on-failure")]
  OnFailure,
  #[serde(rename = "unless-stopped")]
  UnlessStopped,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ServiceVolumeType {
  Volume,
  Bind,
  Tmpfs,
  Npipe,
}
impl Default for ServiceVolumeType {
  fn default() -> Self {
    ServiceVolumeType::Bind
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ServiceVolumeConsistency {
  Consistent,
  Cached,
  Delegated,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServiceVolumeTmpfs {
  size: i64,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct ServiceVolume {
  #[serde(rename = "type")]
  volume_type: ServiceVolumeType,
  source: Option<String>,
  target: Option<String>,
  read_only: Option<bool>,
  tmpfs: Option<ServiceVolumeTmpfs>,
  consistency: Option<ServiceVolumeConsistency>,
}

impl<'de> serde::Deserialize<'de> for ServiceVolume {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    struct ServiceVolumeVisitor;
    impl<'de> serde::de::Visitor<'de> for ServiceVolumeVisitor {
      type Value = ServiceVolume;

      fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Service Volume: https://docs.docker.com/compose/compose-file/#volumes")
      }

      fn visit_str<E>(self, value: &str) -> Result<ServiceVolume, E>
      where
        E: serde::de::Error,
      {
        // TODO: Convert short to long form
        Ok(ServiceVolume {
          source: Some(value.to_string()),
          ..Default::default()
        })
      }
    }
    // Instantiate our Visitor and ask the Deserializer to drive
    // it over the input data, resulting in an instance of MyMap.
    deserializer.deserialize_any(ServiceVolumeVisitor)
  }
}

// Port
// "items": {
//   "oneOf": [
//     {
//       "type": "number",
//       "format": "ports"
//     },
//     {
//       "type": "string",
//       "format": "ports"
//     },
//     {
//       "type": "object",
//       "properties": {
//         "mode": {
//           "type": "string"
//         },
//         "target": {
//           "type": "integer"
//         },
//         "published": {
//           "type": "integer"
//         },
//         "protocol": {
//           "type": "string"
//         }
//       },
//   ]
// },

/*
Sample
version: "3.6.0"
services:
  taiga-back:
    build:
      context: .
      dockerfile: taiga-back.Dockerfile
    working_dir: /src/
    command:
      - "python3"
      - "manage.py"
      - "runserver"
      - "0.0.0.0:9080"
    depends_on:
      - postgres
    ports:
      - target: 9080
        published: 9080
        protocol: tcp
        mode: host
    volumes:
      - ~/Foundry/Panama/installers/taiga/build/src/taiga-back:/src

*/
