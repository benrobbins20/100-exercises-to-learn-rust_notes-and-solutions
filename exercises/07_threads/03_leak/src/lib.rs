// TODO: Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::{thread, vec};

pub fn sum(v: Vec<i32>) -> i32 {
    // alloc the whole vec into heap first
    let v = Box::new(v);
    let v: &'static mut Vec<i32> = Box::leak(v);

    // or just do leak on Vec
    // let v = v.leak();

    let mid = v.len()/2;
    let (a,b) = v.split_at(mid);

    // moves are now operating on &'static slices 
    let t1 = thread::spawn(move ||{ a.iter().sum::<i32>() });
    let t2 = thread::spawn(move ||{ b.iter().sum::<i32>() });
    t1.join().unwrap()+t2.join().unwrap() 
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
