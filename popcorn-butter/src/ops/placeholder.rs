use uuid::Uuid;
use std::marker::PhantomData;
use popcorn::buffer::{Buffer, BufferDevice};
use futures::Future;
use futures::future::Shared;

pub struct Placeholder<T: Send + Copy + 'static> {
  uid: Uuid,
  name: String,
  device: BufferDevice,

  _pd: PhantomData<T>
}

impl<T: Send + Copy + 'static> Placeholder<T> {
  pub fn new<TS: ToString>(t: TS) -> Placeholder<T> {
    Placeholder {
      name: t.to_string(),
      _pd: PhantomData { }
    }
  }

  pub fn name(&self) -> &str { &self.name }
}

impl<T: Send + Copy + 'static> Executable for Placeholder<T> {
  type Base = T;

  fn uid(&self) -> &Uuid { &self.uid }

  fn exec_item(&self, ctx: &Context) -> Shared<Box<Future<Item=Buffer<Self::Base>,Error=Error>>> {
  }
}
