use std::fmt;

use futures::Future;

use hardware::Hardware;
use memory::Memory;

pub trait Device {
  type H: Hardware;
  type M: Memory;
  type Error: fmt::Debug + Clone;

  fn id(&self) -> isize;
  fn hardware(&self) -> &Self::H;
  fn alloc_memory(&self, size: usize) -> Result<Self::M, Self::Error>;
  fn sync_from_vec<T: Send + Copy + Sized + 'static>(&self,
                                                     mem: Self::M,
                                                     vec: Vec<T>) -> Box<Future<Item=Self::M,Error=Self::Error>>;

  fn sync_to_vec<T: Send + Copy + Sized + 'static>(&self,
                                                   mem: Self::M) -> Box<Future<Item=(Self::M, Vec<T>),Error=Self::Error>>;
}
