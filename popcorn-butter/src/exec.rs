use std::collections::HashMap;
use std::any::Any;
use futures::Future;
use futures::future::Shared;
use popcorn::buffer::{Buffer, Error};
use uuid::Uuid;

pub struct Context {
  inputs: HashMap<String, Box<Any>>,
  cache: HashMap<Uuid, Box<Any>>
}

pub trait Executable where Self: Sized {
  type Base: Send + Copy + 'static;

  fn uid(&self) -> &Uuid;
  fn exec<'a>(&self, ctx: &'a mut Context) -> Box<Future<Item=Buffer<Self::Base>,Error=Error>>;
}

impl Context {
  fn get_input<Base: Send + Copy + 'static, E: Executable<Base=Base>>(&mut self,
                                                                      name: &str) ->
    Result<Shared<Box<Future<Item=Buffer<Base>,Error=Error>>> {
    }
}
