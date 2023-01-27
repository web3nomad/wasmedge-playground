use sha2::{Sha256, Digest};
use chrono::prelude::*;
use wasmedge_quickjs::*;
use super::workflow::Workflow;

pub struct RelayPayload {
  pub data: String,
}

pub struct Relay {
  pub hash: String, // 64 hex
  pub parent_hash: String, // 64 hex
  pub root_hash: String, // 64 hex

  pub code: String,
  pub payload: Option<RelayPayload>,

  pub owner: String, // 40 hex
  pub value: u64,
}

const CODE_TRIGGER: &str = r#"
;(function() {
  const data = exec(JSON.parse(inputPayload.data));
  return JSON.stringify(data);
})();
"#;

pub fn exec_code(input_payload: &RelayPayload, code: &str) -> String {
  let mut ctx = Context::new();
  let mut obj = ctx.new_object();
  obj.set("data", ctx.new_string(&input_payload.data).into());
  ctx.get_global().set("inputPayload", obj.into());
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
      let mut input_payload = &RelayPayload {
        data: String::from("{}"),  // 默认为空, 如果 parent 不存在的话
      };
      match workflow.relays.get(&self.parent_hash) {
        Some(parent_relay) => {
          input_payload = parent_relay.payload.as_ref().unwrap()
        },
        _ => {},
      }
      let data = exec_code(input_payload, &self.code);
      self.payload = Some(RelayPayload { data })
    } else {
      panic!("AAAaaaaa!!!!")
    }
    self.hash = Relay::generate_hash();
    println!("owner {} | value {}\n  payload: {}\n   parent: {}\n     hash: {}",
      self.owner, self.value, self.payload.as_ref().unwrap().data, self.parent_hash, self.hash);
    self.hash.clone()
  }

  pub fn new(parent_hash: &str, root_hash: &str, code: &str, owner: &str, value: u64) -> Self {
    Self {
      hash: String::default(),
      parent_hash: String::from(parent_hash),
      root_hash: String::from(root_hash),
      code: String::from(code),
      payload: None,
      owner: String::from(owner),
      value: value,
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
