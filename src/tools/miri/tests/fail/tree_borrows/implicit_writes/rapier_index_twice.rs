// Replicates the error from `rapier2d-0.32.0/src/data/graph.rs:141-155`.
//@compile-flags: -Zmiri-tree-borrows -Zmiri-tree-borrows-implicit-writes

fn index_twice(arr: &mut [u8], a: usize, b: usize) -> (&mut u8, &mut u8) {
    unsafe {
        let ar = &mut *(arr.get_unchecked_mut(a) as *mut u8);
        let br = &mut *(arr.get_unchecked_mut(b) as *mut u8);
        (ar, br) //~ ERROR: /reborrow through .* is forbidden/
    }
}

fn main() {
    let mut buf = [0u8; 4];
    let (a, b) = index_twice(&mut buf, 0, 2);
    *a = 1;
    *b = 2;
}
