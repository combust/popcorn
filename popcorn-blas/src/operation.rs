use futures::Future;
use popcorn::buffer::{Buffer, Error};

pub trait DotOperation<T: Copy + Send + 'static> {
  fn bcast_dot(&self,
               shape_a: Buffer<usize>,
               a: Buffer<T>,
               shape_b: Buffer<usize>,
               b: Buffer<T>,
               mut shape_c: Buffer<usize>,
               mut c: Buffer<T>) ->
    Box<Future<Item=(Buffer<usize>, Buffer<T>, // A
                     Buffer<usize>, Buffer<T>, // B
                     Buffer<usize>, Buffer<T>), Error=Error>>; // Result
}
