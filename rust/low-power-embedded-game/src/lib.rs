// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::ops::Rem;

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    (dividend / divisor, dividend % divisor)
}

pub fn evens<T: std::ops::Rem<i32>>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T>
where
    <T as std::ops::Rem<i32>>::Output: PartialEq<i32>,
{
    iter.enumerate().filter(|(i, v)| v % 2 == 0).map(|v| v.1)
}

pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        unimplemented!("implement `fn manhattan`")
    }
}
