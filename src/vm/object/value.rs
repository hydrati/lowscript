use super::{
  Trace, Finalize, native::NativeObject
};


#[derive(Debug, Trace, Finalize)]
pub enum Value {
  Array(Vec<Value>),
  Buffer(Box<[u8]>),
  Int(u64),
  UInt(u64),

  // Int8(i8),
  // Int16(i16),
  // Int32(i32),
  // Int64(i64),
  // Int128(i128),

  // UInt8(u8),
  // UInt16(u16),
  // UInt32(u32),
  // UInt64(u64),
  // UInt128(u128),
  
  Float(f32),
  Double(f64),

  Bool(bool),

  String(String),

  Function(),

  NativeObject(Box<dyn NativeObject>),
}