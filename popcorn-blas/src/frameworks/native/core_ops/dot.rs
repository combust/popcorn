use blas_sys::c::cblas_sdot;

pub trait Dot where Self: Sized {
  fn dot(a: &[Self], b: &[Self]) -> Self;
}

impl Dot for f32 {
  fn dot(a: &[Self], b: &[Self]) -> Self {
    unsafe {
      cblas_sdot(a.len() as i32, a.as_ptr(), 1, b.as_ptr(), 1)
    }
  }
}
