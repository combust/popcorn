use framework::Framework as IFramework;
use backend;

use super::Framework;
use super::Device;

pub struct Backend {
  device: Device
}

impl Backend {
  pub fn default() -> Backend {
    Backend {
      device: Framework::new().default_device()
    }
  }
}

impl backend::Backend<Framework> for Backend {
  fn device(&self) -> &Device { &self.device }
}
