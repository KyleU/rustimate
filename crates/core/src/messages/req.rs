use crate::{Result, ResultExt};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Sent from client to server, this shared model is used for all server communication
#[derive(Debug, Serialize, Deserialize)]
pub enum RequestMessage {
  Ping { v: i64 },
  GetVersion,

  // Session messages
  JoinSession { id: Uuid },
  UpdateSession { name: String, choices: Vec<String> },
  UpdateProfile { name: String },
  AddPoll { q: String },
  UpdatePoll { poll: Uuid, title: Option<String> },
  // SetPollStatus { poll: Uuid, status: PollStatusType },
  SubmitVote { poll: Uuid, vote: String }
}

impl RequestMessage {
  pub fn from_json(s: &str) -> Result<RequestMessage> {
    serde_json::from_str(&s).chain_err(|| format!("Can't decode JSON RequestMessage from [{}]", s))
  }

  pub fn to_json(&self) -> Result<String> {
    serde_json::to_string_pretty(&self).chain_err(|| "Can't encode JSON RequestMessage")
  }

  pub fn from_binary(b: &[u8]) -> Result<RequestMessage> {
    bincode::deserialize(&b).chain_err(|| "Can't decode binary RequestMessage".to_string())
  }

  pub fn to_binary(&self) -> Result<Vec<u8>> {
    bincode::serialize(&self).chain_err(|| "Can't encode binary RequestMessage")
  }
}
