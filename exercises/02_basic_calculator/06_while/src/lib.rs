// Rewrite the factorial function using a `while` loop.
pub fn factorial(mut n: u32) -> u32 {
    // base case n = 0 or n = 1
    // returns 1
    // else result = result * 3, n - 1, result = result * 2, n -1, etc
    // (1*3), (3*2), (return 6)
    let mut result = 1;
    while n > 1 {
        result *= n;
        n -= 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn fact_three() {
        println!("fact 3 = {}", factorial(3));
        assert_eq!(factorial(3), 6);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        println!("factorial(5) = {}", factorial(5));
        assert_eq!(factorial(5), 120);
    }
}
