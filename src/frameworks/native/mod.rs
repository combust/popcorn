mod device;
mod error;
mod hardware;
mod memory;
mod backend;

use futures_cpupool::Builder;
use ::hardware::Hardware as Hware;

pub use self::device::Device;
pub use self::hardware::Hardware;
pub use self::memory::Memory;
pub use self::error::Error;
pub use self::backend::Backend;

use framework::Framework as IFramework;

pub struct Framework { }

impl Framework {
  pub fn default_hardware(&self) -> Hardware {
    Hardware::new()
  }

  pub fn default_device(&self) -> Device {
    self.new_device(&self.default_hardware()).unwrap()
  }
}

impl IFramework for Framework {
  type H = Hardware;
  type D = Device;
  type Error = error::Error;

  fn name() -> &'static str { "native" }

  fn new() -> Self where Self: Sized {
    Framework { }
  }

  fn load_hardwares(&self) -> Result<Vec<Self::H>, Self::Error> {
    let cpu = Hardware::new();
    Ok(vec![cpu])
  }

  fn new_device(&self, hardware: &Self::H) -> Result<Self::D, Self::Error> {
    let mut builder = Builder::new();
    builder.name_prefix(hardware.name());

    Ok(Device::new(hardware.clone(), builder))
  }
}
