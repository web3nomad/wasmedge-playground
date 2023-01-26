use super::workflow::Workflow;
use sha2::{Sha256, Digest};
use chrono::prelude::*;

pub struct RelayPayload {
  pub data: u64,
}

pub struct Relay {
  pub hash: String,
  pub parent_hash: String,
  pub root_hash: String,

  pub payload: Option<RelayPayload>,
  pub value: u64,
}

impl Relay {
  pub fn execute(&mut self, workflow: &Workflow) {
    if let None = self.payload {
      match workflow.relays_store.get(&self.parent_hash) {
        Some(parent_relay) => {
          self.payload = Some(RelayPayload {
            data: parent_relay.payload.as_ref().unwrap().data + 1
          })
        },
        _ => {
          self.payload = Some(RelayPayload {
            data: u64::default(),
          });
        },
      }
    } else {
      panic!("AAAaaaaa!!!!")
    }
  }

  pub fn new(parent_hash: &str, root_hash: &str) -> Self {
    Self {
      hash: String::default(),
      parent_hash: String::from(parent_hash),
      root_hash: String::from(root_hash),
      payload: None,
      value: u64::default(),
    }
  }

  pub fn generate_hash() -> String {
    let timestamp = Utc::now().timestamp_millis() as u64;
    let mut hasher = Sha256::new();
    hasher.update(timestamp.to_ne_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
  }
}
