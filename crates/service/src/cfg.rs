use crate::cache::ConnectionCache;
use crate::files::FileService;
use crate::session::SessionService;

use rustimate_core::ResponseMessage;
use std::sync::{Arc, RwLock};

/// Contains information about the running application
#[derive(Clone, Debug)]
pub struct AppConfig {
  task: String,
  address: String,
  port: u16,
  files: Arc<FileService>,
  connections: Arc<RwLock<ConnectionCache>>,
  session_svc: Arc<RwLock<SessionService>>,
  root_logger: slog::Logger,
  verbose: bool
}

impl AppConfig {
  pub fn new(task: String, address: String, port: u16, cfg_dir: String, root_logger: slog::Logger, verbose: bool) -> AppConfig {
    let files = Arc::new(FileService::new(&cfg_dir, &root_logger));
    AppConfig {
      task,
      address,
      port,
      files: Arc::clone(&files),
      connections: Arc::new(RwLock::new(ConnectionCache::new(&root_logger))),
      session_svc: Arc::new(RwLock::new(SessionService::new(Arc::clone(&files), root_logger.clone()))),
      root_logger,
      verbose
    }
  }

  pub fn task(&self) -> &String {
    &self.task
  }

  pub fn address(&self) -> &String {
    &self.address
  }

  pub fn port(&self) -> u16 {
    self.port
  }

  pub fn files(&self) -> &FileService {
    &self.files
  }

  pub fn connections(&self) -> &RwLock<ConnectionCache> {
    &self.connections
  }

  pub fn send_channel(&self, key: &str, msg: ResponseMessage) {
    self.connections().read().unwrap().send_channel(key, msg);
  }

  pub fn send_connection(&self, id: &uuid::Uuid, msg: ResponseMessage) {
    self.connections().read().unwrap().send_connection(id, msg);
  }

  pub fn session_svc(&self) -> &RwLock<SessionService> {
    &self.session_svc
  }

  pub fn root_logger(&self) -> &slog::Logger {
    &self.root_logger
  }

  pub fn verbose(&self) -> bool {
    self.verbose
  }
}
