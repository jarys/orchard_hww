use static_alloc::Bump;

// panics for heap < 64kb
#[global_allocator]
static A: Bump<[u8; 1 << 16]> = Bump::uninit(); // 8kB heap

#[test]
fn test_alloc_small() {
    extern crate alloc;
    use alloc::vec;
    use alloc::{boxed::Box, vec::Vec};
    let _: Vec<u8> = vec![1, 2, 3];
    let _: Box<usize> = Box::new(4);
}

#[test]
fn test_alloc_big() {
    extern crate alloc;
    use alloc::vec;
    use alloc::vec::Vec;
    let _: Vec<u8> = vec![0; 1024];
}
