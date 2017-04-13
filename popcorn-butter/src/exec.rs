pub struct Context {
}

pub trait Executable {
  type Item;

  fn uid(&self) -> &Uuid;
  fn exec(&self, ctx: &Context) -> Item {
    ctx.cache(self)
  }
  fn exec_item(&self, ctx: &Context) -> Item;
}
