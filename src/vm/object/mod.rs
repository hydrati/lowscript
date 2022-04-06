pub mod value;
pub mod native;
pub mod property;
pub mod prototype;

use self::value::Value;

pub use super::gc::{
  Trace, Finalize, Ref, Cell, RefMut, Gc
};



#[derive(Debug, Trace, Finalize)]
pub struct Object {
  pub data: Value,
}

