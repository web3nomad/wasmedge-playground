// use super::relay::Relay;

pub struct Workflow {
  pub name: String,
  pub root_relay_hash: String,
}

impl Workflow {
  pub fn new(name: &str) -> Workflow {
    let workflow = Workflow {
      name: String::from(name),
      root_relay_hash: String::default(),
    };
    workflow
  }

  pub fn set_root_hash(&mut self, hash: &str) {
    self.root_relay_hash = String::from(hash);
  }
}
