//! A wrapper for the CLI tool pg_basebackup
//!
//! This is used to generate backup files from a running Postgres database
//! https://www.postgresql.org/docs/12/app-pgbasebackup.html

const APP_NAME: &str = "pg_basebackup";
const MODULE_VERSION: &'static str = env!("CARGO_PKG_VERSION");

use super::*;
use anyhow::{Context, Result};
use serde_derive::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct PgBaseBackup {
  pub instance: AppInstance,
  pub parent: Rc<dyn ContainerTrait>,
}

impl PgBaseBackup {
  // TODO: Convert this to async. Spawn the run function off so it can throw events.
  pub fn run(&self, opts: Options) -> Result<String> {
    log::debug!("Running PgBaseBackup - saving to {:#?}", opts.pgdata);

    let msg = opts.to_message(Some(self.instance.clone()))?;
    log::debug!("msg:\n{:#?}", msg);
    self
      .parent
      .forward(self.instance.clone(), msg.get(0).unwrap().clone())
  }
}

impl AppTrait for PgBaseBackup {
  fn get_name(&self) -> String {
    match &self.instance.version {
      Some(ver) => format!("{} ({})", APP_NAME, ver),
      None => format!("{} (Unknown Version)", APP_NAME),
    }
  }

  fn build(instance: AppInstance, parent: Option<Rc<dyn ContainerTrait>>) -> Result<PgBaseBackup> {
    let container: Rc<dyn ContainerTrait> = match parent {
      Some(x) => x,
      None => {
        let shell = Shell::get_local_shell()?;
        shell.running.clone()
      }
    };
    Ok(PgBaseBackup {
      instance: instance.clone(),
      parent: container.clone(),
    })
  }

  /// Knows how to get the version number of the installed app (not the module version)
  fn set_version(&self, _instance: AppInstance) -> Result<AppInstance> {
    unimplemented!()
  }
  /// Figures out how to call the cli using the given container
  fn set_cli(
    &self,
    _instance: AppInstance,
    _container: Rc<dyn ContainerTrait>,
  ) -> Result<AppInstance> {
    unimplemented!()
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
  Run(Options),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionResult {
  Run(String),
}

impl Action {
  pub fn run(&self, backup: PgBaseBackup) -> Result<ActionResult> {
    match self {
      Action::Run(options) => Ok(ActionResult::Run(
        backup
          .run(options.clone())
          .context("PgBaseBackup::Action::Run failed")?,
      )),
    }
  }
}

/// All the command line options that can be passed to the program
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Options {
  // Options controlling the output:
  ///   -D, --pgdata=DIRECTORY receive base backup into directory
  pgdata: Option<String>,

  ///  -r, --max-rate=RATE    maximum transfer rate to transfer data directory
  ///                         (in kB/s, or use suffix "k" or "M")
  max_rate: Option<Rate>,

  ///  -R, --write-recovery-conf
  ///                         write recovery.conf for replication
  write_recovery_conf: bool,

  ///  -T, --tablespace-mapping=OLDDIR=NEWDIR
  ///                         relocate tablespace in OLDDIR to NEWDIR
  tablespace_mapping: Option<String>,

  ///      --waldir=WALDIR    location for the write-ahead log directory
  waldir: Option<String>,

  ///  -X, --wal-method=none|fetch|stream    include required WAL files with specified method
  wal_method: Option<WalMethod>,

  /// How much and what type of compression should be done on the output
  ///  -F, --format=p|t       output format (plain (default), tar)
  ///  -z, --gzip             compress tar output
  ///  -Z, --compress=0-9     compress tar output with given compression level
  compression: Option<Compression>,

  // General options:
  ///  -c, --checkpoint=fast|spread   set fast or spread checkpointing
  checkpoint: Option<Checkpoint>,

  ///  -C, --create-slot      create replication slot
  create_slot: bool,

  ///  -l, --label=LABEL      set backup label
  label: Option<String>,

  ///  -n, --no-clean         do not clean up after errors
  no_clean: bool,

  ///  -N, --no-sync          do not wait for changes to be written safely to disk
  no_sync: bool,

  ///  -P, --progress         show progress information
  progress: bool,

  ///  -S, --slot=SLOTNAME    replication slot to use
  slot: Option<String>,

  ///  -v, --verbose          output verbose messages
  verbose: bool,

  ///  -V, --version          output version information, then exit
  version: Option<bool>,

  ///      --no-slot          prevent creation of temporary replication slot
  no_slot: bool,

  ///      --no-verify-checksums     do not verify checksums
  no_verify_checksums: bool,

  // TODO: This should be in the postgres instance. Current target is local, so I'm leaving it for later
  ///Connection options:
  ///  -d, --dbname=CONNSTR   connection string

  ///  -h, --host=HOSTNAME    database server host or socket directory

  ///  -p, --port=PORT        database server port number

  ///  -s, --status-interval=INTERVAL
  ///                         time between status packets sent to server (in seconds)

  ///  -U, --username=NAME    connect as specified database user
  username: Option<String>,

  ///  -w, --no-password      never prompt for password
  no_password: bool,

  ///  -W, --password         force password prompt (should happen automatically)
  password: bool,
}

impl Options {
  pub fn new(path: String) -> Options {
    Options {
      pgdata: Some(path),
      ..Default::default()
    }
  }
}

/// The encoding of the output file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Compression {
  None,
  Tar,
  Gzip(u8),
}

impl Default for Compression {
  fn default() -> Self {
    Compression::None
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Rate {
  /// Kilobytes per second
  KbS(i32),
  /// Megabytes per second
  MbS(i32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WalMethod {
  None,
  Fetch,
  Stream,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Checkpoint {
  Fast,
  Spread,
}

impl ActionTrait for Options {
  type RESPONSE = ActionResult;

  fn run(&self, _target: AppInstance) -> Result<Self::RESPONSE> {
    // Should always forward to the parent. How does
    unimplemented!("Still haven't figured out")
  }

  fn to_message(&self, target: Option<AppInstance>) -> Result<Vec<Message>> {
    let mut args = vec![];

    match &self.pgdata {
      None => (),
      Some(x) => {
        args.push("-D".to_string());
        args.push(x.clone());
      }
    };

    let cmd = Message::Command(Cmd {
      run_as: Some("postgres".to_string()),
      command: target.unwrap().get_command_path()?.clone(),
      args,
    });

    Ok(vec![cmd])
  }
}
