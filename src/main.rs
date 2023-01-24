use std::env;

use crate::models::relay::Relay;
use crate::models::workflow::Workflow;

mod models;

fn handle_relay() {
  let mut workflow = Workflow::new("first workflow");
  let mut root: Relay = Relay::new_root();
  workflow.set_root_hash(&root.hash);
  root.set_workflow(&workflow);
  let mut relay1 = Relay::new(&root);
  let relay2 = Relay::new(&relay1);
  relay1.execute();
  println!("workflow | name: {}", workflow.name);
  println!("root | value: {}\n  hash: {}", root.payload.as_ref().unwrap().data, root.hash);
  println!("relay1 | value: {}\n  hash: {}\n  parent: {}", relay1.payload.as_ref().unwrap().data, relay1.hash, relay1.parent_hash);
  println!("relay2 | value: {}\n  hash: {}\n  parent: {}", relay2.payload.as_ref().unwrap().data, relay2.hash, relay2.parent_hash);
}

fn main() {
  println!("args:");
  for argument in env::args().skip(1) {
    println!("- {}", argument);
  }
  // handle_relayer();
  handle_relay();
}
