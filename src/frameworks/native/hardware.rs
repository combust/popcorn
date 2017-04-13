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
  /// Returns the name of the Hardware
  fn name(&self) -> &str { &self.name }

  /// Returns the device_type of the Hardware
  fn hardware_type(&self) -> hardware::HardwareType { hardware::HardwareType::CPU }

  /// Returns the compute_units of the Hardware
  fn compute_units(&self) -> usize { self.compute_units }
}
