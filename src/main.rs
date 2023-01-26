use std::env;

use crate::models::relay::Relay;
use crate::models::workflow::Workflow;

mod models;
// mod lifetime;

fn handle_relay() {
  let mut workflow = Workflow::new("first workflow");
  println!("workflow | name: {}", workflow.name);

  let mut root: Relay = Relay::new(&String::default(), &String::default());
  root.execute(&workflow);
  let hash = Relay::generate_hash();
  root.hash = hash.clone();
  println!("root | value: {}\n  hash: {}\n  parent: {}", root.payload.as_ref().unwrap().data, root.hash, root.parent_hash);
  workflow.relays_store.insert(root.hash.clone(), root);
  workflow.root_relay_hash = hash.clone();

  let mut relay1 = Relay::new(&hash, &workflow.root_relay_hash);
  relay1.execute(&workflow);
  let hash = Relay::generate_hash();
  relay1.hash = hash.clone();
  println!("relay1 | value: {}\n  hash: {}\n  parent: {}", relay1.payload.as_ref().unwrap().data, relay1.hash, relay1.parent_hash);
  workflow.relays_store.insert(relay1.hash.clone(), relay1);

  let mut relay2 = Relay::new(&hash, &workflow.root_relay_hash);
  relay2.execute(&workflow);
  // relay2.execute(&workflow);
  let hash = Relay::generate_hash();
  relay2.hash = hash.clone();
  println!("relay2 | value: {}\n  hash: {}\n  parent: {}", relay2.payload.as_ref().unwrap().data, relay2.hash, relay2.parent_hash);
  workflow.relays_store.insert(relay2.hash.clone(), relay2);
}

fn main() {
  println!("args:");
  for argument in env::args().skip(1) {
    println!("- {}", argument);
  }
  // handle_relayer();
  handle_relay();
  // crate::lifetime::run();
}
