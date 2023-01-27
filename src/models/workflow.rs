use std::collections::HashMap;
use wasmedge_quickjs::*;
use super::relay::{Relay, RelayPayload};

pub struct Workflow {
  pub name: String,

  pub test_code: String,

  pub root_relay_hash: String,
  pub final_relay_hash: String,
  pub relays: HashMap<String, Relay>,
}

const TEST_CODE_TRIGGER: &str = r#"
;(function() {
  const data = test(JSON.parse(relayPayload.data));
  return data;
})();
"#;

pub fn exec_test_code(relay_payload: &RelayPayload, test_code: &str) -> bool {
  let mut ctx = Context::new();
  let mut obj = ctx.new_object();
  obj.set("data", ctx.new_string(&relay_payload.data).into());
  ctx.get_global().set("relayPayload", obj.into());
  let mut code = String::from(test_code);
  code.push_str(TEST_CODE_TRIGGER);
  let r = ctx.eval_global_str(&code);
  if let JsValue::Bool(val) = r {
    return val;
  } else {
    panic!("Oooops");
  }
}

impl Workflow {
  pub fn new(name: &str, test_code: &str) -> Workflow {
    let workflow = Workflow {
      name: String::from(name),
      test_code: String::from(test_code),
      root_relay_hash: String::default(),
      final_relay_hash: String::default(),
      relays: HashMap::new(),
    };
    workflow
  }

  pub fn add_relay(&mut self, relay: Relay, root: bool) {
    if root {
      self.root_relay_hash = relay.hash.clone();
    }
    self.check_relay_payload(&relay);
    self.relays.insert(relay.hash.clone(), relay);
    // TODO: 1. check root hash, 2. check payload
  }

  pub fn check_relay_payload(&mut self, relay: &Relay) {
    if let Some(relay_payload) = &relay.payload {
      let result = exec_test_code(&relay_payload, &self.test_code);
      if result {
        self.final_relay_hash = relay.hash.clone();
        println!("---\nTest passed!\n     hash: {}\n---", self.final_relay_hash);
      }
    } else {
      panic!("Something went wrong!!!!")
    }
  }
}
