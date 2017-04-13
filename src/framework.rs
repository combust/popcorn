use hardware::Hardware;
use device::Device;
use std::fmt;

/// Defines a Framework.
pub trait Framework {
  /// The Hardware representation for this Framework.
  type H: Hardware;

  /// The Device representation for this Framework.
  type D: Device + Clone;

  /// The Error representation for this Framework
  type Error: fmt::Debug + Clone;

  /// Defines the Framework by a Name.
  fn name() -> &'static str;

  /// Initializes a new Framework.
  fn new() -> Self where Self: Sized;

  /// Initializes all the available hardwares.
  fn load_hardwares(&self) -> Result<Vec<Self::H>, Self::Error>;

  /// Initializes a new Device from the provided hardware.
  fn new_device(&self, &Self::H) -> Result<Self::D, Self::Error>;
}
