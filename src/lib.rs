extern crate futures;
extern crate futures_cpupool;

pub mod backend;
pub mod hardware;
pub mod framework;
pub mod memory;
pub mod device;
pub mod buffer;
pub mod frameworks;

pub use backend::Backend;
pub use hardware::Hardware;
pub use framework::Framework;
pub use memory::Memory;
pub use device::Device;
pub use buffer::{Buffer, BufferDevice};

pub use frameworks::native;

#[cfg(test)]
mod test {
  use super::*;
  use futures::Future;

  #[test]
  #[cfg(feature = "native")]
  fn test_native() {
    let backend = native::Backend::default();
    let dev = backend.device();
    let buf1: Buffer<f64> = Buffer::new(dev, 4).unwrap();
    let f1 = buf1.sync_from_vec(vec![23.0, 45.5, 54.2, 42.0], dev);
    let f2 = f1.and_then(|x| x.sync_to_vec(dev));
    let (_, nv) = f2.wait().unwrap();

    println!("HELLO: {:?}", nv);
  }
}
