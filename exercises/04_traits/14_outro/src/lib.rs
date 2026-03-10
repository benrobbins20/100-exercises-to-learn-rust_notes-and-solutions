// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::ops::Add;

#[derive(Debug,PartialEq, Clone, Copy)]
pub struct SaturatingU16 {
    val: u16,
}

// add another paritaleq to 'decapsulate' Satu16 to u16
impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.val == *other
    }
}

// Sat16 + Sat16, default condition more or less
impl Add for SaturatingU16 {
    type Output = Self; // Sat Add
    fn add(self, rhs: Self) -> Self::Output {
        SaturatingU16 {val: self.val.saturating_add(rhs.val.into())}
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = Self;
    fn add(self, rhs: u16) -> Self::Output {
        SaturatingU16 { val: self.val.add(rhs)}
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = Self;
    fn add(self, rhs: &SaturatingU16) -> Self::Output {
        let sat_u16 = *rhs;
        self + sat_u16 // 
    }
}

impl SaturatingU16 {
    pub fn new(val: u16) -> Self {
        SaturatingU16 { val }
    }
}

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        SaturatingU16 { val: value }
    }
}

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        SaturatingU16 { val: value as u16 }
    }
}


impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        let raw_u8 = *value; // first deref the u8 and then delegate back to the u8 impl above
        SaturatingU16 { val: raw_u8.into() }
    }
}

// same for &u16
impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        let raw_u16 = *value;
        SaturatingU16 { val: raw_u16 }
    }
} 

