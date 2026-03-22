fn fill_vec(n: usize) -> Vec<u32>{
    let mut v: Vec<u32> = Vec::new();
    v.push(0);
    v.push(1);

    for i in 2..=n {
        v.push(v[i-1]+v[i-2])
    }

    v
}

pub fn fibonacci(n: u32) -> u32 {
    match n {
        // handle base cases
        0 => 0,
        1 => 1,

        // anything else, fill the vector with n fibs and return the last
        _ => {
            let v = fill_vec(n as usize);
            *v.last().unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::fibonacci;

    #[test]
    fn first() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn second() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn tenth() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn thirtieth() {
        assert_eq!(fibonacci(30), 832040);
    }
}
