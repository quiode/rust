// Replicates the read pattern from `input_buffer-0.5.0/src/lib.rs:155-176`.
// Gets spare capacity, zeroes it via raw pointer, creates a byte slice, reads into it.
// Causes problems in: `tungstenite`, `actix-http`.
//@compile-flags: -Zmiri-tree-borrows -Zmiri-tree-borrows-implicit-writes
//@error-in-other-file: /reborrow through .* is forbidden/

use std::io::Read;

fn fill_buf(buf: &mut Vec<u8>, stream: &mut impl Read) -> usize {
    buf.reserve(64);
    unsafe {
        let spare = buf.spare_capacity_mut();
        // input_buffer zeros the memory before reading to avoid UB from uninitialized reads
        let ptr = spare.as_mut_ptr().cast::<u8>();
        for i in 0..spare.len() {
            ptr.add(i).write(0);
        }
        // then creates a byte slice from the same raw pointer and reads into it
        let data = std::slice::from_raw_parts_mut(ptr, spare.len());
        let n = stream.read(data).unwrap();
        buf.set_len(buf.len() + n);
        n
    }
}

fn main() {
    let mut buf = Vec::new();
    let mut stream = std::io::Cursor::new(b"hello");
    fill_buf(&mut buf, &mut stream);
    assert_eq!(&buf, b"hello");
}
