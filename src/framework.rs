use hardware::Hardware;
use device::Device;
use std::fmt;

pub trait Framework {
  type H: Hardware;
  type D: Device + Clone;
  type Error: fmt::Debug + Clone;

  fn name() -> &'static str;
  fn new() -> Self where Self: Sized;
  fn load_hardwares(&self) -> Result<Vec<Self::H>, Self::Error>;
  fn new_device(&self, &Self::H) -> Result<Self::D, Self::Error>;
}
