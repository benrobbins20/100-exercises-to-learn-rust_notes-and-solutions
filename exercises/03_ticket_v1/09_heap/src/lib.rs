// strings are vectors so expandable sections in heap memory
// the stack will store a pointer to the location in heap memory
// 64 bit arch will store 8 bytes for pointer, 8 bytes for length (used space), 8 bytes for capacity (allocated space)
pub struct Ticket <'a>{
    title: String, // 24 bytes
    description: String, // 24 bytes
    status: String, // 24 bytes
    test_str: &'a str, // references include lifetime specifier, 'a, which essentially says Ticket and test_str live the same lifetime, 16 bytes (fat pointer)
    test_string: &'a String, // should be a pointer to pointer, so 8 bytes (thin pointer)
}
#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    #[test]
    fn string_size() {
        assert_eq!(size_of::<String>(), 24);
    }

    #[test]
    fn str_size() {
        assert_eq!(size_of::<&str>(), 16);
    }

    #[test]
    fn string_ref_size() {
        assert_eq!(size_of::<&String>(), 8);
    }

    #[test]
    fn ticket_size() {
        // ticket happens to be 3 strings, 72 bytes, some types can be larger than 24 bytes
        // https://doc.rust-lang.org/reference/type-layout.html for more information.
        assert_eq!(size_of::<Ticket>(), 96);
    }
}
