extern crate futures;
extern crate popcorn;
extern crate blas_sys;

pub mod operation;
pub mod frameworks;

pub use operation::*;
pub use frameworks::native::*;

#[cfg(test)]
mod native_test {
  use popcorn;
  use popcorn::*;
  use super::*;
  use futures::Future;

  #[test]
  fn dot_test() {
    let backend = popcorn::frameworks::native::Backend::default();

    let shape_vec = vec![1, 4];

    let shape_a: Buffer<usize> = Buffer::new(backend.device(), 2).unwrap().sync_from_vec(shape_vec.clone(), backend.device()).wait().unwrap();
    let a: Buffer<f32> = Buffer::new(backend.device(), 4).unwrap().sync_from_vec(vec![1.0, 2.0, 3.0, 4.0], backend.device()).wait().unwrap();
    let shape_b: Buffer<usize> = Buffer::new(backend.device(), 2).unwrap().sync_from_vec(shape_vec.clone(), backend.device()).wait().unwrap();
    let b: Buffer<f32> = Buffer::new(backend.device(), 4).unwrap().sync_from_vec(vec![2.0, 2.0, 2.0, 2.0], backend.device()).wait().unwrap();
    let shape_c: Buffer<usize> = Buffer::new(backend.device(), 0).unwrap();
    let c: Buffer<f32> = Buffer::new(backend.device(), 1).unwrap();

    let (_shape_aa, _aa, _shape_bb, _bb, shape_c, c) = backend.bcast_dot(shape_a, a,
                                                                         shape_b, b,
                                                                         shape_c, c).wait().unwrap();

    let shape_c_vec = shape_c.native_memory(backend.device()).unwrap().try_as_slice::<usize>().unwrap();
    let c_vec = c.native_memory(backend.device()).unwrap().try_as_slice::<f32>().unwrap();
    println!("Shape: {:?}", &shape_c_vec);
    println!("Contents: {:?}", &c_vec);
  }
}
