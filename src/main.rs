use std::env;

use crate::models::relay::Relay;
use crate::models::workflow::Workflow;

mod models;
// mod jscode;
// mod lifetime;
// mod block;

const CODE0: &str = r#"
function exec() {
  return {
    a: 0,
    b: 0,
  }
}
"#;

const CODE1: &str = r#"
function exec({ a, b }) {
  return {
    a: a + 1,
    b: b + 1,
    c: 10
  }
}
"#;

const CODE2: &str = r#"
function exec({ a, b, c }) {
  return {
    a: a + 2,
    b: b + 2,
    c: c + 2
  }
}
"#;

fn handle_relay() {
  let mut workflow = Workflow::new("first workflow");
  println!("workflow | name: {}", workflow.name);

  let mut root: Relay = Relay::new(&String::default(), &String::default(), CODE0);
  let hash = root.execute(&workflow);
  workflow.add_relay(root, true);

  let mut relay1 = Relay::new(&hash, &workflow.root_relay_hash, CODE1);
  let hash = relay1.execute(&workflow);
  workflow.add_relay(relay1, false);

  let mut relay2 = Relay::new(&hash, &workflow.root_relay_hash, CODE2);
  relay2.execute(&workflow);
  workflow.add_relay(relay2, false);
}

fn main() {
  println!("args:");
  for argument in env::args().skip(1) {
    println!("- {}", argument);
  }
  handle_relay();
  // crate::jscode::execute();
  // crate::lifetime::run();
}
