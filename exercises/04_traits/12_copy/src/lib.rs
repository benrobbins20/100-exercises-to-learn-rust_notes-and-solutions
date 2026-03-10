// TODO: implement the necessary traits to make the test compile and pass.
//  You *can't* modify the test.

use std::{ops::Add};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct WrappingU32 {
    value: u32,
}

impl WrappingU32 {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

impl Add for WrappingU32 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        // same is Self::new, more explicit in my head
        WrappingU32::new(self.value.wrapping_add(rhs.value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops() {
        let x = WrappingU32::new(42);
        let y = WrappingU32::new(31);
        let z = WrappingU32::new(u32::MAX);
        /* 
            this is misleading because the error initially is that there is no add impl
            you need to first impl add in general so you can sum the numbers but the point
            is that y needs to be copy because y is used twice. it needs to handle the move
        */

        assert_eq!(x + y + y + z, WrappingU32::new(103));
    }
}
