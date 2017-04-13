pub mod shape;
pub mod iter;

use std::cmp;
pub use self::shape::*;
pub use self::iter::{DenseStrideIter, DenseBroadcastIter};

use popcorn::buffer::Error;

pub fn try_new_broadcast<'a, T: 'a>(shape_a: &[usize],
                                    a: &'a [T],
                                    shape_b: &[usize],
                                    b: &'a [T],
                                    chop: usize) -> Result<(Vec<usize>, DenseBroadcastIter<'a, T>, DenseBroadcastIter<'a, T>), Error> {
  if !compatible(shape_a, shape_b) {
    return Err(Error::InvalidBroadcast)
  }

  let mut bshape = target_shape(shape_a, shape_b);
  let strides_a = DenseStrideIter::new(shape_a);
  let strides_b = DenseStrideIter::new(shape_b);

  let mut bdims_a = BroadcastDimension::shape_from_iters(shape_a.iter().map(|x| *x),
  bshape.iter().map(|x| *x),
  strides_a);

  let mut bdims_b = BroadcastDimension::shape_from_iters(shape_b.iter().map(|x| *x),
  bshape.iter().map(|x| *x),
  strides_b);

  let len = bdims_a.len();
  bdims_a.truncate(cmp::max(1, len - chop));
  bdims_b.truncate(cmp::max(1, len - chop));
  bshape.truncate(cmp::max(1, len - chop));

  let iter_a = DenseBroadcastIter::new(bdims_a, a);
  let iter_b = DenseBroadcastIter::new(bdims_b, b);

  Ok((bshape, iter_a, iter_b))
}
