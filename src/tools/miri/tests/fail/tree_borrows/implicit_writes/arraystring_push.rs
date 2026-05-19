// Replicates the problem from `arraystring-0.3.0/src/arraystring.rs:670-681`.
//@compile-flags: -Zmiri-tree-borrows -Zmiri-tree-borrows-implicit-writes

use std::ptr::copy_nonoverlapping;

struct FixedBuf {
    data: [u8; 32],
    size: u8,
}

impl FixedBuf {
    fn new() -> Self {
        FixedBuf { data: [0u8; 32], size: 0 }
    }

    fn len(&self) -> usize {
        self.size as usize
    }

    // mirroring arraystring's `as_mut_bytes` which calls `get_unchecked_mut(..len)`.
    unsafe fn as_mut_bytes(&mut self) -> &mut [u8] {
        let len = self.len();
        self.data.get_unchecked_mut(..len)
    }

    // mirrors `push_str_unchecked`
    unsafe fn push_str(&mut self, s: &str) {
        let (src, slen) = (s.as_ptr(), s.len());
        let dest = self.as_mut_bytes().as_mut_ptr().add(self.len());
        copy_nonoverlapping(src, dest, slen); //~ ERROR: /write access through .* is forbidden/
        self.size += slen as u8;
    }
}

fn main() {
    let mut buf = FixedBuf::new();
    unsafe { buf.push_str("hello") };
    assert_eq!(&buf.data[..5], b"hello");
}
