use std::result::Result;
use std::mem;
use std::ptr;
use std::slice;

use super::Error;
use memory;

#[derive(Debug, Clone)]
pub struct Memory {
  buf: Box<[u8]>
}

impl Memory {
  pub fn alloc(size: usize) -> Memory {
    let vec: Vec<u8> = vec![0; size];
    let bx: Box<[u8]> = vec.into_boxed_slice();

    Memory {
      buf: bx
    }
  }

  pub fn len(&self) -> usize { self.buf.len() }

  pub fn as_ptr(&self) -> *const u8 {
    self.buf.as_ptr()
  }

  pub fn as_mut_ptr(&mut self) -> *mut u8 {
    self.buf.as_mut_ptr()
  }

  pub fn try_as_slice<T: Sized + Copy>(&self) -> Result<&[T], Error> {
    if self.len() % mem::size_of::<T>() != 0 {
      // TODO: better error
      return Err(Error::OutOfMemory)
    }

    unsafe {
      let p = self.as_ptr();
      let pt = mem::transmute::<*const u8, *const T>(p);
      Ok(slice::from_raw_parts(pt, self.len() / mem::size_of::<T>()))
    }
  }

  pub fn try_as_mut_slice<T: Sized + Copy>(&mut self) -> Result<&mut [T], Error> {
    if self.len() % mem::size_of::<T>() != 0 {
      // TODO: better error
      return Err(Error::OutOfMemory)
    }

    unsafe {
      let p = self.as_mut_ptr();
      let pt = mem::transmute::<*mut u8, *mut T>(p);
      Ok(slice::from_raw_parts_mut(pt, self.len() / mem::size_of::<T>()))
    }
  }

  pub fn copy_from<T: Sized + Copy>(&mut self,
                                    vs: &[T]) -> Result<(), Error> {
    if self.len() != vs.len() * mem::size_of::<T>() {
      // TODO: better error
      return Err(Error::OutOfMemory)
    }

    unsafe {
      let p = vs.as_ptr();
      let bytes = mem::transmute::<*const T, *const u8>(p);
      ptr::copy_nonoverlapping(bytes, self.as_mut_ptr(), self.buf.len());
      Ok(())
    }
  }

  pub fn into_vec<T: Sized + Copy>(self) -> Result<Vec<T>, Error> {
    if self.len() % mem::size_of::<T>() != 0 {
      // TODO: better error
      return Err(Error::OutOfMemory)
    }

    unsafe {
      let mut buf = self.buf.clone();
      let buf_ptr = mem::transmute::<*mut u8, *mut T>(buf.as_mut_ptr());
      let len = self.len() / mem::size_of::<T>();
      let vec = Vec::from_raw_parts(buf_ptr, len, len);
      mem::forget(buf);
      Ok(vec)
    }
  }
}

impl memory::Memory for Memory { }
