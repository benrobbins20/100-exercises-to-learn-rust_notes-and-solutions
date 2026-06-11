use std::thread::scope;

pub fn sum(v: Vec<i32>) -> i32 {
    let mid = v.len()/2;
    scope(|scope| {

        // in scope 'static requirement is unnessecary, v guarenteed to not be in use in this function after scope
        let t1 = scope.spawn(|| { // move unnessesary, borrow is implicit
            v[0..mid].iter().sum::<i32>()
        });
        let t2 = scope.spawn(|| {
            v[mid..].iter().sum::<i32>()
        });

        // return i32 in scope which returns 
        t1.join().unwrap() + t2.join().unwrap()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
