use std::collections::HashMap;
use super::relay::Relay;

pub struct Workflow {
  pub name: String,
  pub root_relay_hash: String,
  pub relays_store: HashMap<String, Relay>,
}

impl Workflow {
  pub fn new(name: &str) -> Workflow {
    let workflow = Workflow {
      name: String::from(name),
      root_relay_hash: String::default(),
      relays_store: HashMap::new(),
    };
    workflow
  }

  // pub fn set_root_hash(&mut self, hash: &str) {
  //   self.root_relay_hash = String::from(hash);
  // }
}
