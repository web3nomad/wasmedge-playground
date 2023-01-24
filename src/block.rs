use std::time::Instant;

pub struct RelayPayload {
  pub data: u32,
  pub value: u32,
}

pub struct Relayer {
  name: String,
  code: u32,
}

impl Relayer {
  pub fn new(name: &str) -> Relayer {
    return Relayer {
      name: String::from(name),
      code: 0,
    }
  }

  pub fn execute(payload: &RelayPayload) -> RelayPayload {
    // input_data -> algorithm -> output_data
    let current_timestamp = Instant::now().elapsed().as_millis() as u32;
    let result: RelayPayload = RelayPayload {
      data: current_timestamp,
      value: payload.value / 2,
    };
    return result;
  }

  pub fn print(&self) {
    println!("{} {}", self.name, self.code);
  }
}
