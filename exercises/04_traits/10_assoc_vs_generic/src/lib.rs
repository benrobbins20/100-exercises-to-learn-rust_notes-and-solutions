// TODO: Define a new trait, `Power`, that has a method `power` that raises `self`
//  to the power of `n`.
//  The trait definition and its implementations should be enough to get
//  the tests to compile and pass.
//
// Recommendation: you may be tempted to write a generic implementation to handle
// all cases at once. However, this is fairly complicated and requires the use of
// additional crates (i.e. `num-traits`).
// Even then, it might be preferable to use a simple macro instead to avoid
// the complexity of a highly generic implementation. Check out the
// "Little book of Rust macros" (https://veykril.github.io/tlborm/) if you're
// interested in learning more about it.
// You don't have to though: it's perfectly okay to write three separate
// implementations manually. Venture further only if you're curious.


// create a generic trait with a method power which only takes a reference to &self
// method signature 'contract'. Like a C header declaration
// this is like Deref style, it takes a generic <Exponent = Self> and associated Output type
pub trait Power<E = Self> {
    // Power::Output is the exclusive type that power returns
    type Output;
    // power method takes &self to call .power, and n exponent generic type argument
    fn power(&self, n: E) -> Self::Output;
}

// provide custom implementation to convert multple int types to u32 which power will return
// u16 kind of on its own because it's the most different
impl Power<u16> for u32 {
    type Output = u32;

    // expand the u16 impl into u32
    fn power(&self, n: u16) -> Self::Output {
        self.pow(n as u32)
    }
}

impl Power<u32> for u32 {
    type Output = u32;
    fn power(&self, n: u32) -> Self::Output {
        self.pow(n)
    }
}

impl Power<&u32> for u32 {
    type Output = u32;
    fn power(&self, n: &u32) -> Self::Output {
        // if you deref the &u32, it's the same as u32 implementation, reuse
        self.power(*n)
    }
}


#[cfg(test)]
mod tests {
    use super::Power;

    #[test]
    fn test_power_u16() {
        let x: u32 = 2_u32.power(3u16);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_ref_u32() {
        let x: u32 = 2_u32.power(&3u32);
        assert_eq!(x, 8);
    }
}
