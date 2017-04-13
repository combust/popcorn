#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub enum HardwareType {
  CPU,
  GPU,
  ACCELERATOR,
  OTHER
}

pub trait Hardware {
  fn name(&self) -> &str;
  fn hardware_type(&self) -> HardwareType;
  fn compute_units(&self) -> usize;
}
