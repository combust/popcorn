pub mod broadcast;
pub mod core_ops;

use popcorn::frameworks::native::Framework;
use popcorn::backend::Backend;
use operation::*;
use futures::Future;
use popcorn::buffer::{Buffer, BufferDevice, Error};
use self::core_ops::*;

impl<T: Dot + Copy + Sized + Send + 'static> DotOperation<T> for Backend<Framework> {
  fn bcast_dot(&self,
               shape_a: Buffer<usize>,
               a: Buffer<T>,
               shape_b: Buffer<usize>,
               b: Buffer<T>,
               shape_c: Buffer<usize>,
               c: Buffer<T>) ->
    Box<Future<Item=(Buffer<usize>, Buffer<T>,
                     Buffer<usize>, Buffer<T>,
                     Buffer<usize>, Buffer<T>), Error=Error>> {
      // Step 1. Sync all input buffers to the required device
      let bdev = BufferDevice::Native(self.device().clone());
      let ar = shape_a.sync(&bdev).join(a.sync(&bdev));
      let br = shape_b.sync(&bdev).join(b.sync(&bdev));
      let cr = shape_c.sync(&bdev).join(c.sync(&bdev));

      let dev = self.device().clone();
      let pool = self.device().pool().clone();

      Box::new(ar.join(br).join(cr).and_then(move |(((shape_a, a), (shape_b, b)), (mut shape_c, mut c))| {
        pool.spawn_fn(move || {
          {
            let n_shape_a: &[usize] = try!(try!(shape_a.native_memory(&dev)).try_as_slice());
            let n_a: &[T] = try!(try!(a.native_memory(&dev)).try_as_slice());
            let n_shape_b: &[usize] = try!(try!(shape_b.native_memory(&dev)).try_as_slice());
            let n_b: &[T] = try!(try!(b.native_memory(&dev)).try_as_slice());
            let n_shape_c: &mut [usize] = try!(try!(shape_c.native_memory_mut(&dev)).try_as_mut_slice());
            let n_c: &mut [T] = try!(try!(c.native_memory_mut(&dev)).try_as_mut_slice());

            let n_shape_a_vec = &n_shape_a[..n_shape_a.len() - 1];
            let n_shape_b_vec = &n_shape_b[..n_shape_b.len() - 1];
            let (mut bshape, iter_a, iter_b) = try!(broadcast::try_new_broadcast(n_shape_a_vec, n_a, n_shape_b_vec, n_b));
            bshape.push(1);
            let r_iter = iter_a.zip(iter_b).map(|(a, b)| T::dot(a, b));

            for (v1, v2) in bshape.iter().zip(n_shape_c.iter_mut()) {
              *v2 = *v1;
            }

            for(v1, v2) in r_iter.zip(n_c.iter_mut()) {
              *v2 = v1;
            }
          }

          Ok((shape_a, a,
              shape_b, b,
              shape_c, c))
        })
      }))
    }
}
