// Replicates the error from `into_out`.
// `tail` gets a write in `into_out`, even though it's unused afterwards. This creates UB even though it should be fine.
//@compile-flags: -Zmiri-tree-borrows -Zmiri-tree-borrows-implicit-writes

struct Buf<'a> {
    out_ptr: *mut u8,
    #[allow(unused)]
    tail: &'a mut u8,
}

impl<'a> Buf<'a> {
    fn into_out(self) {
        let _ = unsafe { std::slice::from_raw_parts(self.out_ptr, 4) }; //~ ERROR: /reborrow through .* is forbidden/
    }
}

fn main() {
    let mut buf = [0u8; 4];
    let ptr = buf.as_mut_ptr();
    let tail = unsafe { &mut *ptr.add(3) };
    Buf { out_ptr: ptr, tail }.into_out();
}
