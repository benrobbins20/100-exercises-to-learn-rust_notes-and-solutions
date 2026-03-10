// TODO: implement a so-called "Drop bomb": a type that panics when dropped
//  unless a certain operation has been performed on it.
//  You can see the expected API in the tests below.

struct DropBomb {
    defused: bool
}
impl DropBomb {
    pub fn new() -> Self {
        DropBomb { defused: false }
    }
    pub fn defuse(&mut self) {
        self.defused = true;
    }
}

// basically the lesson here is that you must consider other held references, memory, or other conditions before dropping
// custom drop implementations may be used
impl Drop for DropBomb {
    fn drop(&mut self) {
        if !self.defused {
            panic!("boom")
        }
        else {
            println!("dropped normally");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_drop_bomb() {
        let _bomb = DropBomb::new();
        // The bomb should panic when dropped without 'clean up' method defuse()
    }

    // after test is finished it'll still run the drop routine routine
    // log message printed demonstrates that 'instances' will drop when going out of scope
    #[test]
    fn test_defused_drop_bomb() {
        let mut bomb = DropBomb::new();
        bomb.defuse();
        // The bomb should not panic when dropped
        // since it has been defused
    }
}
