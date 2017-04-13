use futures::{Future, IntoFuture};
use std::marker::PhantomData;
use std::mem;
use std::collections::HashMap;
use device::Device;

use frameworks::native;

pub type BBF<T> = Box<Future<Item=Buffer<T>, Error=Error>>;

#[derive(Debug, Clone, Copy)]
pub enum BufferSource {
  #[cfg(feature = "native")]
  Native
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum BufferDevice {
  #[cfg(feature = "native")]
  Native(native::Device)
}

#[derive(Debug)]
pub enum BufferMemory {
  #[cfg(feature = "native")]
  Native(native::Memory)
}

#[derive(Debug, Clone)]
pub enum Error {
  #[cfg(feature = "native")]
  Native(native::Error),

  InvalidDevice,
  InvalidBroadcast
}

#[cfg(feature = "native")]
impl From<native::Error> for Error {
  fn from(err: native::Error) -> Error { Error::Native(err) }
}

#[derive(Debug)]
/// Container that handles synchronization of [Memory][1] of type `T`.
/// [1]: ../memory/index.html
pub struct Buffer<T: Copy + Sized + Send + 'static> {
  /// Size of this buffer. Because we don't store
  /// The contents in a slice, this information
  /// Needs to be saved separately from the memory
  size: usize,

  /// Copies of this Buffer on different devices
  /// Does not include the latest copy
  copies: HashMap<BufferDevice, BufferMemory>,

  /// Latest copy of the memory, this is paired with the
  /// latest_event field to indicate when this memory is ready
  latest_source: BufferSource,

  _pd: PhantomData<T>,
}

impl<T: Send + Copy + Sized + 'static> Buffer<T> {
  pub fn new(dev: &BufferDevice, size: usize) -> Result<Buffer<T>, Error> {
    let mut copies = HashMap::new();
    let copy = try!(Self::alloc_on_device(dev, size * mem::size_of::<T>()));
    copies.insert(dev.clone(), copy);

    Ok(Buffer {
      size: size,
      copies: copies,
      latest_source: Self::device_source(dev),
      _pd: PhantomData
    })
  }

  fn alloc_on_device(dev: &BufferDevice, size: usize) -> Result<BufferMemory, Error> {
    match *dev {
      #[cfg(feature = "native")]
      BufferDevice::Native(ref dev_n) => Self::alloc_on_device_native(dev_n, size),
    }
  }

  fn alloc_on_device_native(dev: &native::Device, size: usize) -> Result<BufferMemory, Error> {
    dev.alloc_memory(size).
      map(|m| BufferMemory::Native(m)).
      map_err(|e| Error::Native(e))
  }

  fn source(&self) -> BufferSource { self.latest_source }

  fn device_source(dev: &BufferDevice) -> BufferSource {
    match *dev {
      #[cfg(feature = "native")]
      BufferDevice::Native(_) => BufferSource::Native,
    }
  }

  pub fn sync_from_vec(mut self, vec: Vec<T>, dev: &BufferDevice) -> Box<Future<Item=Buffer<T>,Error=Error>> {
    let copy = self.copies.remove(dev);
    match copy {
      Some(mem) => {
        match *dev {
          #[cfg(feature = "native")]
          BufferDevice::Native(ref dev) => {
            let BufferMemory::Native(m) = mem;
            let new_dev = BufferDevice::Native(dev.clone());
            Box::new(dev.sync_from_vec(m, vec).map(move |mem| {
              self.latest_source = Self::device_source(&new_dev);
              self.copies.insert(new_dev, BufferMemory::Native(mem));
              self
            }).map_err(Error::Native))
          },
        }
      },
      None => Err(Error::InvalidDevice).into_future().boxed()
    }
  }

  pub fn sync_to_vec(mut self, dev: &BufferDevice) -> Box<Future<Item=(Buffer<T>, Vec<T>),Error=Error>> {
    let copy = self.copies.remove(dev);
    match copy {
      Some(mem) => {
        match *dev {
          #[cfg(feature = "native")]
          BufferDevice::Native(ref dev) => {
            let BufferMemory::Native(m) = mem;
            let new_dev = BufferDevice::Native(dev.clone());
            Box::new(dev.sync_to_vec(m).map(move |(mem, vec)| {
              self.copies.insert(new_dev, BufferMemory::Native(mem));
              (self, vec)
            }).map_err(Error::Native))
          },
        }
      },
      None => Err(Error::InvalidDevice).into_future().boxed()
    }
  }

  pub fn sync(self, dev: &BufferDevice) -> Box<Future<Item=Buffer<T>,Error=Error>> {
    match self.source() {
      #[cfg(feature = "native")]
      BufferSource::Native => {
        match *dev {
          #[cfg(feature = "native")]
          BufferDevice::Native(_) => Box::new(Ok(self).into_future()),
        }
      },
    }
  }

  #[cfg(feature = "native")]
  pub fn native_memory(&self, dev: &native::Device) -> Result<&native::Memory, Error> {
    match self.copies.get(&BufferDevice::Native(dev.clone())) {
      Some(mem) => {
        let BufferMemory::Native(ref nm) = *mem;
        Ok(nm)
      },
      None => Err(Error::InvalidDevice)
    }
  }

  #[cfg(feature = "native")]
  pub fn native_memory_mut(&mut self, dev: &native::Device) -> Result<&mut native::Memory, Error> {
    match self.copies.get_mut(&BufferDevice::Native(dev.clone())) {
      Some(mem) => {
        let BufferMemory::Native(ref mut nm) = *mem;
        Ok(nm)
      },
      None => Err(Error::InvalidDevice)
    }
  }
}
