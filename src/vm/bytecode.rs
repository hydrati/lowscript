#[derive(Debug)]
pub enum OpCode {
  /**
   * R = Registers,
   * K = Constants,
   * P = Closure Prototype,
   * U = Captured
   */

  /// (), (noop)
  Noop,

  /// (A, B), R[A] = R[B]
  Move(u16, u16),

  /// (A, B), R[A] = K[B]
  LoadConst(u16, u16),

  /// (A, B, T), R[A] = (T)K[B]
  LoadConstAs(u16, u16, Type),

  /// (A), R[A] = true
  LoadBoolTrue(u16),

  /// (A), R[A] = false
  LoadBoolFalse(u16),

  /// (A), R[A] = false; pc += 1
  LoadBoolFalseSkip(u16),

  /// (A, B), R[A] = (Int,size)B
  LoadInt(u16, i64),

  /// (A, B), R[A] = (UInt,size)B
  LoadUInt(u16, u64),

  // /// (A, B), R[A] = (Int8)B
  // LoadInt8(u16, i8),

  // /// (A, B), R[A] = (Int16)B
  // LoadInt16(u16, i16),

  // /// (A, B), R[A] = (Int32)B
  // LoadInt32(u16, i32),

  // /// (A, B), R[A] = (Int64)B
  // LoadInt64(u16, i64),

  // /// (A, B), R[A] = (Int128)B
  // LoadInt128(u16, i128),

  // /// (A, B), R[A] = (UInt8)B
  // LoadUInt8(u16, u8),

  // /// (A, B), R[A] = (UInt16)B
  // LoadUInt16(u16, u16),

  // /// (A, B), R[A] = (UInt32)B
  // LoadUInt32(u16, u32),

  // /// (A, B), R[A] = (UInt64)B
  // LoadUInt64(u16, u64),

  // /// (A, B), R[A] = (UInt128)B
  // LoadUInt128(u16, u128),

    
  // /// (A, B), R[A] = (Float32)B
  // LoadFloat32(u16, f32),

  /// (A, B), R[A] = (Float)B
  LoadFloat(u16, f64),

  /// (A, B), R[A] = (Char)B
  LoadChar(u16, char),


  /// (A, B, C), R[A] = R[B] + R[C]
  Add(u16, u16, u16),

  /// (A, B, C), R[A] = R[B] - R[C]
  Sub(u16, u16, u16),

  /// (A, B, C), R[A] = R[B] * R[C]
  Mul(u16, u16, u16),

  /// (A, B, C), R[A] = R[B] % R[C]
  Mod(u16, u16, u16),

  /// (A, B, C), R[A] = R[B] ^ R[C]
  Pow(u16, u16, u16),

  /// (A, B, C), R[A] = R[B] / R[C]
  Div(u16, u16, u16),

  /// (A, B, C), R[A] = R[B] // R[C]
  IDiv(u16, u16, u16),

  

  /// (A, B, C), R[A] = R[B] & R[C]
  BitAnd(u16, u16, u16),

  /// (A, B, C), R[A] = R[B] | R[C]
  BitOr(u16, u16, u16),

  /// (A, B, C), R[A] = R[B] ~ R[C]
  BitXor(u16, u16, u16),

  /// (A, B, C), R[A] = R[B] << R[C]
  BitShiftL(u16, u16, u16),

  /// (A, B, C), R[A] = R[B] >> R[C]
  BitShiftR(u16, u16, u16),

  /// (A, B), R[A] = ~R[B]
  BitNot(u16, u16),

  /// (A, B), R[A] = -R[B]
  Negete(u16, u16),

  /// (A, B), R[A] = not R[B]
  Not(u16, u16),

  /// (A, B, C), if (R[B] == C) R[A] = R[B] else pc += 1
  AssertAssign(u16, u16, bool),

  /// (A, B, k), if ((R[A] == R[B]) == !k) then pc += 1
  Equal(u16, u16, bool),

  /// (A, B, k), if ((R[A] == K[B]) == !k) then pc += 1 (skip next)
  EqualK(u16, u16, bool),


  /// (A, B, k), if ((R[A] < R[B]) == !k) then pc += 1
  Less(u16, u16, bool),

  /// (A, B, k), if ((R[A] <= R[B]) == !k) then pc += 1
  LessEqual(u16, u16, bool),

  /// (A, B, k), if ((R[A] > R[B]) == !k) then pc += 1
  More(u16, u16, bool),

  /// (A, B, k), if ((R[A] >= R[B]) == !k) then pc += 1
  MoreEqual(u16, u16, bool),

  /// (A, B, k), if ((R[A] < K[B]) == !k) then pc += 1
  LessK(u16, u16, bool),

  /// (A, B, k), if ((R[A] <= K[B]) == !k) then pc += 1
  LessEqualK(u16, u16, bool),

  /// (A, B, k), if ((R[A] > K[B]) == !k) then pc += 1
  MoreK(u16, u16, bool),

  /// (A, B, k), if ((R[A] >= K[B]) == !k) then pc += 1
  MoreEqualK(u16, u16, bool),

  /// (i), pc += i
  Jump(usize),

  /// (A, B, C), R[A] = R[B](R[B+1], ..., R[B+C-1])
  CallR1(u16, u16, u16),

  /// (A, B, C), R[A] = R[B](R[C])
  Call1R1(u16, u16, u16),

  /// (A, B), R[A](R[A]+1, ..., R[A+B-1])
  CallR0(u16, u16),

  /// (A), R[A]()
  Call0R0(u16),

  /// (A, B), R[A] = R[B]()
  Call0R1(u16, u16),

  /// (A, B, C) R[A], ..., R[A+C-2] = R[A](R[A+1], ..., R[A+B-1])
  Call(u16, u16, u16),

  /// (A, B), return R[A](R[A+1], ..., R[A+B-1])
  TailCall(u16, u16),

  /// (A, B), return R[A](R[B])
  TailCall1(u16, u16),

  /// (A), return R[A]
  Return1(u16),

  /// (A), return K[A]
  Return1K(u16),

  /// (), return
  Return0,

  /// (A, B), return R[A], ..., R[A+B-1]
  Return(u16, u16),

  // /// (A), [extra] = R[A]
  // /// _R = R(copy ptr)
  // /// R = [R[A], R[A][0], R[A][1]....]
  // SetExtraArg(u16),

  // /// (), R = _R
  // CleanExtraArg(),

  /// (A, B, C), R[A][R[B]] = K[C]
  SetFieldK(u16, u16, u16),

  /// (A, B, C), R[A][K[B]] = K[C]
  SetFieldK2(u16, u16, u16),

  /// (A, B, C), R[A][R[B]] = R[C]
  SetField(u16, u16, u16),

  /// (A, B, C), R[A][(usize)B] = R[C]
  SetI(u16, usize, u16),

  /// (A, B, C), R[A] = R[B][R[C]]
  GetField(u16, u16, u16),

  /// (A, B, C), R[A] = R[B][K[C]]
  GetFieldK(u16, u16, u16),

  /// (A, B, C), R[A] = R[B][(usize)B]
  GetI(u16, usize, u16),

  /// (A, B), R[A] = P[B]
  Closure(u16, u16),

  /// (A, B), R[A] = U[B]
  GetCaptured(u16, u16),

  /// (A, B), U[B] = R[A]
  SetCaptured(u16, u16),

  /// (A, B), R[A] = [PARENT SCOPE].find(K[B])
  GetCapturedParent(u16, u16),

  /// (A, B), [PARENT SCOPE].find(K[B]) = R[A]
  SetCapturedParent(u16, u16),

  /// (A, B), R[A] = len(R[B])
  LengthOf(u16, u16),

  /// (A, B), R[A] = len(K[B])
  LengthOfK(u16, u16),

  /// (A, B, T), R[A] = [(usize)B]{}
  AllocArray(u16, usize),

  /// (A), R[A] = {}
  AllocObject(u16),

  /// (A, B, C), R[A] = new R[B](R[B+1], ..., R[B+C-1])
  Construct(u16, u16, u16),

  /// (A), R[A] = null
  LoadNull(u16),

  /// (A, k), if ((R[A] == null) != k) then pc += 1
  EqualNull(u16, bool),

  /// (A, B, C, D), R[A], R[B] = R[C][R[D]](self = R[B]), &R[C]
  LoadMethod(u16, u16, u16, u16),

  /// (A, B, C, D), R[A], R[B] = R[C][K[D]](self = R[B]), &R[C]
  LoadMethodK(u16, u16, u16, u16),

  
}

#[derive(Debug, PartialEq)]
pub enum Type {
}

// #[derive(Debug, PartialEq)]
// pub struct Operation {
//   code: OpCode
// }


// impl Operation {
//   pub fn new(code: OpCode) -> Operation {
//     Operation { code }
//   }
// }


// #[cfg(test)]
// mod tests {
//   use super::*;

//   #[test]
//   fn test_create_hlt() {
//     let code = OpCode::HLT;
//     assert_eq!(code, OpCode::HLT);
//   }

//   #[test]
//   fn test_create_op() {
//     let op = Operation::new(OpCode::HLT);
//     assert_eq!(op.code, OpCode::HLT);
//   }
// }