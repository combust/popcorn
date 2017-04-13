use std::fmt;

use futures::Future;

use hardware::Hardware;
use memory::Memory;

/// Specifies Hardware behavior across frameworks.
pub trait Device {
  /// The Hardware representation for this Device.
  type H: Hardware;

  /// The Memory representation for this Device.
  type M: Memory;

  /// The Error representation for this Device.
  type Error: fmt::Debug + Clone;

  /// Returns the unique identifier of the Device.
  fn id(&self) -> isize;

  /// Returns the hardware, which defines the Device.
  fn hardware(&self) -> &Self::H;

  /// Allocates a new buffer on the Device.
  fn alloc_memory(&self, size: usize) -> Result<Self::M, Self::Error>;

  /// Sync data from Vec into memory
  fn sync_from_vec<T: Send + Copy + Sized + 'static>(&self,
                                                     mem: Self::M,
                                                     vec: Vec<T>) -> Box<Future<Item=Self::M,Error=Self::Error>>;

  /// Sync data from device to Vec
  fn sync_to_vec<T: Send + Copy + Sized + 'static>(&self,
                                                   mem: Self::M) -> Box<Future<Item=(Self::M, Vec<T>),Error=Self::Error>>;
}
