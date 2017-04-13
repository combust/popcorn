use std::sync::Arc;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Copy)]
pub enum Error {
  AlreadyLocked
}

pub struct Lock<T> {
  // Should not use underlying Arc eventually?
  arc: Arc<T>
}

pub struct LockGuard<T> {
  arc: Arc<T>
}

impl<T> Lock<T> {
  pub fn new(data: T) -> Lock<T> {
    Lock {
      arc: Arc::new(data)
    }
  }

  pub fn try_lock(&self) -> Result<LockGuard<T>, Error> {
    if Arc::strong_count(&self.arc) == 1 {
      Ok(LockGuard {
        arc: self.arc.clone()
      })
    } else {
      Err(Error::AlreadyLocked)
    }
  }
}

impl<T> Deref for LockGuard<T> {
  type Target = T;

  fn deref(&self) -> &T { &self.arc }
}

impl<T> DerefMut for LockGuard<T> {
  fn deref_mut(&mut self) -> &mut T { Arc::get_mut(&mut self.arc).unwrap() }
}
