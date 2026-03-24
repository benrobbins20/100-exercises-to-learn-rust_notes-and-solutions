// TODO: Define a function named `sum` that takes a reference to a slice of `u32` and returns the sum of all
//  elements in the slice.

use std::{any::type_name, slice::Iter};



// so silly, can't just print the types after compiling
// so it takes a generic, and type_name takes the context of the generic
fn print_type<T>(_:&T) {
    // type name works on types, so supplying it with type ::<T> (comes from the parameter)
    println!("{}", type_name::<T>());
}

fn sum(nums: &[u32]) -> u32{
    let iter: Iter<'_,u32> = nums.iter();
    // print_type(&iter); // core::slice::iter::Iter<'_, u32>

    let mut s = 0;
    for n in iter {
        s += n;
    }
    
    s

    // nums.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn sum_test() {
    //     let nums = vec![1,2,3];
    //     sum(&nums);
    // }

    #[test]
    fn empty() {
        let v = vec![];
        assert_eq!(sum(&v), 0);
    }

    #[test]
    fn one_element() {
        let v = vec![1];
        assert_eq!(sum(&v), 1);
    }

    #[test]
    fn multiple_elements() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(sum(&v), 15);
    }

    #[test]
    fn array_slice() {
        let v = [1, 2, 3, 4, 5];
        assert_eq!(sum(&v), 15);
    }
}
