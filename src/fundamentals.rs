use saturn_tagging::bit_utils;
use crate::gc::GcPtr;

#[allow(non_camel_case_types)]
#[repr(transparent)]
/// A signed integer with space reserved for a tag component. Mostly
/// just works like a regular integer, but in debug builds there's
/// code that checks the width of these things.
///
/// NOTE: this is actually a 49-bit signed integer, and can store at
/// most a 48-bit unsigned integer. Calling it an `i48` breaks Rust
/// convention, but it feels more appropriate than `i49`.
pub struct i48(i64);

impl From<i64> for i48 {
    fn from(i: i64) -> i48 {
        bit_utils::assert_is_clean(i as _);
        i48(i)
    }
}

impl From<i48> for i64 {
    fn from(i48(i): i48) -> i64 { i }
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// A unified value type, addressing unit, and raw memory
/// constituient. Any "chunk of memory" is a `[Cell]`; any pointer
/// will be `Cell`-aligned.
pub struct Cell(u64);

#[repr(transparent)]
pub struct Atom(GcPtr<str>);

#[repr(align(16))]
pub struct Pair {
    head: Cell,
    tail: Cell,
}

#[repr(transparent)]
pub struct Cons(GcPtr<Pair>);

/// An expanded tagged cell
pub enum Value {
    Integer(i48),
    Cons(Cons),
    Atom(Atom),
    Float(f64),
}
