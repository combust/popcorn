use hardware;

#[derive(Debug, Clone)]
pub struct Hardware {
  name: String,
  compute_units: usize
}

impl Hardware {
  pub fn new() -> Hardware {
    Hardware {
      name: "cpu".to_string(),
      compute_units: 1
    }
  }
}

impl hardware::Hardware for Hardware {
  fn name(&self) -> &str { &self.name }
  fn hardware_type(&self) -> hardware::HardwareType { hardware::HardwareType::CPU }
  fn compute_units(&self) -> usize { self.compute_units }
}
