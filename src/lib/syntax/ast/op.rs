use std::fmt::{Formatter, Result, Display};
use syntax::ast::op::UnaryOp::*;
use syntax::ast::expr::ExprDef::*;
use syntax::ast::op::CompOp::*;
use syntax::ast::op::BitOp::*;
use syntax::ast::op::NumOp::*;
use syntax::ast::op::BinOp::*;
use syntax::ast::keyword::Keyword::*;
use syntax::ast::op::BinOp::*;
/// Represents an operator
pub trait Operator {
    /// Get the associativity as a boolean that is true if it goes rightwards
    fn get_assoc(&self) -> bool;
    /// Get the precedence as an unsigned integer, where the lower it is, the more precedence/priority it has
    fn get_precedence(&self) -> u64;
    #[inline(always)]
    /// Get the precedence and associativity of this operator
    fn get_precedence_and_assoc(&self) -> (u64, bool) {
        (self.get_precedence(), self.get_assoc())
    }
}
#[derive(Clone, PartialEq)]
/// A numeric operation between 2 values
pub enum NumOp {
    /// `a + b` - Addition
    OpAdd,
    /// `a - b` - Subtraction
    OpSub,
    /// `a / b` - Division
    OpDiv,
    /// `a * b` - Multiplication
    OpMul,
    /// `a % b` - Modulus
    OpMod
}
impl Display for NumOp {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", match *self {
            OpAdd => "+",
            OpSub => "-",
            OpDiv => "/",
            OpMul => "*",
            OpMod => "%"
        })
    }
}
#[derive(Clone, PartialEq)]
/// A unary operation on a single value
pub enum UnaryOp {
    /// `a++` - increment the value
    UnaryIncrementPost,
    /// `++a` - increment the value
    UnaryIncrementPre,
    /// `a--` - decrement the value
    UnaryDecrementPost,
    /// `--a` - decrement the value
    UnaryDecrementPre,
    /// `-a` - negate the value
    UnaryMinus,
    /// `+a` - convert to a number
    UnaryPlus,
    /// `!a` - get the opposite of the boolean value
    UnaryNot
}
impl Display for UnaryOp {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", match *self {
            UnaryIncrementPost | UnaryIncrementPre => "++",
            UnaryDecrementPost | UnaryDecrementPre => "--",
            UnaryPlus => "+",
            UnaryMinus => "-",
            UnaryNot => "!"
        })
    }
}
#[derive(Clone, PartialEq)]
/// A bitwise operation between 2 values
pub enum BitOp {
    /// `a & b` - Bitwise and
    BitAnd,
    /// `a | b` - Bitwise or
    BitOr,
    /// `a ^ b` - Bitwise xor
    BitXor,
    /// `a << b` - Bit-shift leftwards
    BitShl,
    /// `a >> b` - Bit-shift rightrights
    BitShr
}
impl Display for BitOp {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", match *self {
            BitAnd => "&",
            BitOr => "|",
            BitXor => "^",
            BitShl => "<<",
            BitShr => ">>"
        })
    }
}
#[derive(Clone, PartialEq)]
/// A comparitive operation between 2 values
pub enum CompOp {
    /// `a == b` - Equality
    CompEqual,
    /// `a != b` - Unequality
    CompNotEqual,
    /// `a === b` - Strict equality
    CompStrictEqual,
    /// `a !== b` - Strict unequality
    CompStrictNotEqual,
    /// `a > b` - If `a` is greater than `b`
    CompGreaterThan,
    /// `a >= b` - If `a` is greater than or equal to `b`
    CompGreaterThanOrEqual,
    /// `a < b` - If `a` is less than `b`
    CompLessThan,
    /// `a <= b` - If `a` is less than or equal to `b`
    CompLessThanOrEqual,
}
impl Display for CompOp {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", match *self {
            CompEqual => "==",
            CompNotEqual => "!=",
            CompStrictEqual => "===",
            CompStrictNotEqual => "!==",
            CompGreaterThan => ">",
            CompGreaterThanOrEqual => ">=",
            CompLessThan => "<",
            CompLessThanOrEqual => "<="
        })
    }
}
#[derive(Clone, PartialEq)]
/// A logical operation between 2 boolean values
pub enum LogOp {
    /// `a && b` - Logical and
    LogAnd,
    /// `a || b` - Logical or
    LogOr
}
impl Display for LogOp {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", match *self {
            LogAnd => "&&",
            LogOr => "||"
        })
    }
}
#[derive(Clone, PartialEq)]
/// A binary operation between 2 values
pub enum BinOp {
    /// Numeric operation
    BinNum(NumOp),
    /// Bitwise operation
    BinBit(BitOp),
    /// Comparitive operation
    BinComp(CompOp),
    /// Logical operation
    BinLog(LogOp)
}
impl Operator for BinOp {
    fn get_assoc(&self) -> bool {
        true
    }
    fn get_precedence(&self) -> u64 {
        match *self {
            BinNum(OpMul) | BinNum(OpDiv) | BinNum(OpMod) => 5,
            BinNum(OpAdd) | BinNum(OpSub) => 6,
            BinBit(BitShl) | BinBit(BitShr) => 7,
            BinComp(CompLessThan) | BinComp(CompLessThanOrEqual) | BinComp(CompGreaterThan) | BinComp(CompGreaterThanOrEqual) => 8,
            BinComp(CompEqual) | BinComp(CompNotEqual) | BinComp(CompStrictEqual) | BinComp(CompStrictNotEqual) => 9,
            BinBit(BitAnd) => 10,
            BinBit(BitXor) => 11,
            BinBit(BitOr) => 12,
            BinLog(LogAnd) => 13,
            BinLog(LogOr) => 14,

        }
    }
}
impl Display for BinOp {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", match *self {
            BinNum(op) => op.to_string(),
            BinBit(op) => op.to_string(),
            BinComp(op) => op.to_string(),
            BinLog(op) => op.to_string()
        })
    }
}
