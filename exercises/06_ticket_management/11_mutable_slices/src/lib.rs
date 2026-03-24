// TODO: Define a function named `squared` that raises all `i32`s within a slice to the power of 2.
//  The slice should be modified in place.

// dump function that can piece together 8 byte memory chunks
fn dump(nums: &[i32]) {
    let ptr = nums.as_ptr();
    let len = nums.len() * size_of::<i32>();

    let bytes = unsafe {
        // from_raw_parts<'a, T>(data: *const T, len: usize) -> &'a [T]
        std::slice::from_raw_parts(
            ptr as *const u8, // cast as a constant pointer to bytes
            len,
        )
    };

    println!("nums at {:?}", ptr);
    for b in bytes {
        print!("{:02x} ", b);
    }
    println!();
}


// pass it a mutable slice
fn squared(nums: &mut [i32]) {
    for n in nums {
        *n = n.pow(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut s = vec![];
        println!("s: {:p}",s.as_ptr());
        squared(&mut s);
        assert_eq!(s, vec![]);
    }

    #[test]
    fn one() {
        let mut s = [2];
        println!("s: {:p}",s.as_ptr()); // extracts stack memory location
        squared(&mut s);
        println!("s: {:p}",s.as_ptr());
        assert_eq!(s, [4]);
    }

    #[test]
    fn multiple() {
        let mut s = vec![2, 3, 4];
        // println!("&s (Vec on stack): {:p}", &s);
        // println!("s: {:p}",s.as_ptr()); // extracts the heap ptr from the vec struct
        dump(&s);
        squared(&mut s); 
        dump(&s);
        // set breakpoint here, in debug `memory read -c 12 0xMEM_ADDR`
        // it shows the squared memory addresses
        assert_eq!(s, vec![4, 9, 16]); 
    }
}
