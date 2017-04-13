use std::sync::Arc;

use futures::Future;
use futures_cpupool::{CpuPool, Builder};

use device;
use super::Hardware;
use super::Memory;
use super::Error;
use std::hash::{Hash, Hasher};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Device {
  id: isize,
  inner: Arc<Inner>
}

struct Inner {
  hardware: Hardware,
  pool: CpuPool
}

impl Device {
  pub fn new(hardware: Hardware, mut builder: Builder) -> Device {
    let inner = Arc::new(Inner {
      hardware: hardware,
      pool: builder.create()
    });

    Device {
      id: 0,
      inner: inner
    }
  }

  pub fn pool(&self) -> &CpuPool {
    &self.inner.pool
  }
}

impl device::Device for Device {
  type H = Hardware;
  type M = Memory;
  type Error = Error;

  fn id(&self) -> isize { self.id }
  fn hardware(&self) -> &Self::H { &self.inner.hardware }
  fn alloc_memory(&self, size: usize) -> Result<Self::M, Self::Error> {
    Ok(Memory::alloc(size))
  }

  fn sync_from_vec<T: Send + Copy + Sized + 'static>(&self,
                                                     mut mem: Self::M,
                                                     vec: Vec<T>) -> Box<Future<Item=Self::M,Error=Self::Error>> {
    Box::new(self.inner.pool.spawn_fn(move || {
      try!(mem.copy_from(&vec));
      Ok(mem)
    }))
  }

  fn sync_to_vec<T: Send + Copy + Sized + 'static>(&self,
                                                   mem: Self::M) -> Box<Future<Item=(Self::M, Vec<T>),Error=Self::Error>> {
    Box::new(self.inner.pool.spawn_fn(move || {
      let vec: Vec<T> = try!(mem.clone().into_vec());
      Ok((mem, vec))
    }))
  }
}

impl PartialEq for Device {
  fn eq(&self, o: &Self) -> bool {
    self.id == o.id
  }
}

impl Eq for Device { }

impl Hash for Device {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.id.hash(state);
  }
}

impl fmt::Debug for Inner {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Inner {{ hardware: {:?} }}", &self.hardware)
  }
}
