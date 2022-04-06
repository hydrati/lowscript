use std::any::Any;
use core::fmt::Debug;
use super::Trace;

pub trait NativeObject: Debug + Any + Trace {
  fn as_any(&self) -> &dyn Any;
  fn as_mut_any(&mut self) -> &mut dyn Any;
}

impl <T: Any + Debug + Trace> NativeObject for T {
  #[inline]
  fn as_any(&self) -> &dyn Any {
    self as &dyn Any
  }

  #[inline]
  fn as_mut_any(&mut self) -> &mut dyn Any {
    self as &mut dyn Any
  }
}

