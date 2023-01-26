use sha2::{Sha256, Digest};
use chrono::prelude::*;
use wasmedge_quickjs::*;
use super::workflow::Workflow;

pub struct RelayPayload {
  pub data: String,
}

pub struct Relay {
  pub hash: String,
  pub parent_hash: String,
  pub root_hash: String,

  pub code: String,
  pub payload: Option<RelayPayload>,
  pub value: u64,
}

const CODE_TRIGGER: &str = r#"
;(function() {
  const data = exec(JSON.parse(input_payload.data));
  return JSON.stringify(data);
})();
"#;

pub fn exec_code(input_payload: &RelayPayload, code: &str) -> String {
  let mut ctx = Context::new();
  let mut obj = ctx.new_object();
  obj.set("data", ctx.new_string(&input_payload.data).into());
  ctx.get_global().set("input_payload", obj.into());
  let mut code = String::from(code);
  code.push_str(CODE_TRIGGER);
  let r = ctx.eval_global_str(&code);
  if let JsValue::String(val) = r {
    return val.to_string();
  } else {
    panic!("Oooops");
  }
}

impl Relay {
  pub fn execute(&mut self, workflow: &Workflow) -> String {
    if let None = self.payload {
      let mut input_payload: &RelayPayload = &RelayPayload {
        data: String::from("{}"),
      };
      match workflow.relays_store.get(&self.parent_hash) {
        Some(parent_relay) => {
          input_payload = parent_relay.payload.as_ref().unwrap()
        },
        _ => {
          //
        },
      }
      let data = exec_code(input_payload, &self.code);
      self.payload = Some(RelayPayload { data })
    } else {
      panic!("AAAaaaaa!!!!")
    }
    self.hash = Relay::generate_hash();
    println!("relay | payload: {}\n  hash: {}\n  parent: {}", self.payload.as_ref().unwrap().data, self.hash, self.parent_hash);
    self.hash.clone()
  }

  pub fn new(parent_hash: &str, root_hash: &str, code: &str) -> Self {
    Self {
      hash: String::default(),
      parent_hash: String::from(parent_hash),
      root_hash: String::from(root_hash),
      code: String::from(code),
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
