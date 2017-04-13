use std::cmp;
use std::iter::repeat;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BroadcastDimension {
  // Size of the slice
  pub size: usize,

  // Stride for each iteration
  // This will be 0 for broadcasted dimensions
  // Or if all dimensions are sparse and it is not needed
  pub stride: usize,

  // Target number of iterations for this dimension
  pub target: usize
}

impl BroadcastDimension {
  pub fn shape_from_iters<I1: Iterator<Item=usize>,
  I2: Iterator<Item=usize>,
  I3: Iterator<Item=usize>>(shape: I1,
                            bshape: I2,
                            strides: I3) -> Vec<BroadcastDimension> {
    shape.zip(bshape).zip(strides).map(|((a, b), s)| {
      let stride = if a == 1 { 0 } else { s };
      let target = cmp::max(a, b);

      BroadcastDimension {
        stride: stride,
        size: s,
        target: target
      }
    }).collect()
  }
}

pub fn compatible<'a>(shape1: &'a [usize],
                      shape2: &'a [usize]) -> bool {
  shape1.iter().rev().zip(shape2.iter().rev()).all(|(&a, &b)| {
    a == b || a == 1 || b == 1
  })
}

pub fn target_shape<'a>(shape1: &'a [usize],
                        shape2: &'a [usize]) -> Vec<usize> {
  let len1 = shape1.len();
  let len2 = shape2.len();

  match len1.cmp(&len2) {
    cmp::Ordering::Equal => {
      shape1.iter().zip(shape2.iter()).
        map(|(&a, &b)| cmp::max(a, b)).
        collect()
    },
    cmp::Ordering::Less => {
      let one: usize = 1;

      repeat(&one).take(len2 - len1).chain(shape1.iter()).zip(shape2.iter()).
        map(|(&a, &b)| cmp::max(a, b)).
        collect()
    },
    cmp::Ordering::Greater => {
      let one: usize = 1;

      shape1.iter().zip(repeat(&one).take(len1 - len2).chain(shape2.iter())).
        map(|(&a, &b)| cmp::max(a, b)).
        collect()
    }
  }
}

