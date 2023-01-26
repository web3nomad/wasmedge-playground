use super::workflow::Workflow;
use sha2::{Sha256, Digest};
use chrono::prelude::*;

pub struct RelayPayload {
  pub data: u64,
}

pub struct Relay<'a> {
  pub workflow: Option<&'a Workflow>,
  pub hash: String,
  pub parent_hash: String,
  pub root_hash: String,
  pub payload: Option<RelayPayload>,
  pub value: u64,
}

impl<'a> Relay<'a> {
  pub fn execute(&mut self) {
    if let None = self.payload {
      self.payload = Some(RelayPayload {
        data: 1,
      })
    } else {
      self.payload = Some(RelayPayload {
        data: self.payload.as_ref().unwrap().data + 1
      })
    }
  }

  pub fn new<'b, 'c:'b>(parent_relay: &'b Relay<'c>) -> Relay<'c> {
    Relay {
      workflow: parent_relay.workflow,
      hash: Self::generate_hash(),
      root_hash: parent_relay.root_hash.clone(),
      parent_hash: parent_relay.hash.clone(),
      payload: None,
      value: u64::default(),
    }
  }

  pub fn new_root() -> Self {
    let root_payload = RelayPayload {
      data: u64::default(),
    };
    let hash = Self::generate_hash();
    Self {
      workflow: None,
      hash: hash.clone(),
      root_hash: hash,
      parent_hash: String::default(),
      payload: Some(root_payload),
      value: u64::default(),
    }
  }

  pub fn set_workflow(&mut self, workflow: &'a Workflow) {
    self.workflow = Some(workflow);
  }

  pub fn generate_hash() -> String {
    let timestamp = Utc::now().timestamp_millis() as u64;
    let mut hasher = Sha256::new();
    hasher.update(timestamp.to_ne_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
  }
}
