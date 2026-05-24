//@compile-flags: -Zmiri-tree-borrows

// In `borrow_tracker/tree_borrows/mod.rs`, if a Box<T> is !Unpin, we leave the function early and return None. This allows this test to pass. If this would not be the case, this test would have UB.

use std::marker::PhantomPinned;

pub struct NotUnpin(i32, PhantomPinned);

fn f(_b: Box<NotUnpin>, xraw: *mut i32) {
    unsafe { *xraw = 42 };
    std::mem::forget(_b);
}

fn main() {
    let mut b = Box::new(NotUnpin(0, PhantomPinned));

    let ptr: *mut NotUnpin = &raw mut *b;
    let xraw: *mut i32 = std::ptr::addr_of_mut!(b.0);

    f(b, xraw);
    assert!(unsafe { *xraw } == 42);
    // Reconstruct the Box to free the allocation that f forgot.
    unsafe { drop(Box::from_raw(ptr)) };
}
