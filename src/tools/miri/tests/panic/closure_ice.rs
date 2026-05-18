//@compile-flags: -Zmiri-tree-borrows -Zmiri-tree-borrows-implicit-writes
//@rustc-env:RUSTC_ICE=0
// Normalize away the backtrace (same as mir-validation.rs)
//@normalize-stderr-test: "\n +[0-9]+:.+" -> ""
//@normalize-stderr-test: "\n +at .+" -> ""
//@normalize-stderr-test: "\n +\[\.\.\. omitted [0-9]+ frames? \.\.\.\].*" -> ""
//@normalize-stderr-test: "\n[ =]*note:.*" -> ""
//@normalize-stderr-test: "/rustc-dev/[^/]*/" -> "/rustc-dev/$HASH/"

// Miri ICE when encountering a `SyntheticCoroutineBody` from an external crate (see
// `closure_ice_lib` for details). Calling the async closure triggers the ICE.

use std::future::Future;
use std::task::{Context, Poll, Waker};

use futures::SinkExt as _;

// Taken from tests/pass/async-fn.rs.
fn run_fut<T>(fut: impl Future<Output = T>) -> T {
    let mut context = Context::from_waker(Waker::noop());
    let mut pinned = Box::pin(fut);
    loop {
        match pinned.as_mut().poll(&mut context) {
            Poll::Pending => continue,
            Poll::Ready(v) => return v,
        }
    }
}

fn main() {
    let mut sink = Box::pin(closure_ice_lib::line_sink());
    run_fut(sink.send(())).unwrap();
}
