//! Provides a representation for a collection of available compute units e.g. CPUs or GPUs.
//!
//! Hardware can be GPUs, multi-core CPUs or DSPs, Cell/B.E. processor or whatever else
//! is supported by the provided frameworks such as OpenCL, CUDA, etc. The struct holds all
//! important information about the hardware.
//! To execute code on hardware, turn hardware into a [device][device].
//!
//! [device]: ../device/index.html

#[derive(Debug, Copy, Clone, PartialEq, Hash)]
/// Specifies the available Hardware types.
pub enum HardwareType {
  /// CPU devices
  CPU,
  /// GPU devices
  GPU,
  /// Hardware Accelerator devices
  ACCELERATOR,
  /// Used for anything else
  OTHER
}

/// Specifies Hardware behavior accross frameworks.
pub trait Hardware {
  /// Returns the name of the Hardware
  fn name(&self) -> &str;

  /// Returns the device_type of the Hardware
  fn hardware_type(&self) -> HardwareType;

  /// Returns the compute_units of the Hardware
  fn compute_units(&self) -> usize;
}
