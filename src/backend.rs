use framework::Framework;

pub trait Backend<F: Framework> {
  fn device(&self) -> &F::D;
}
