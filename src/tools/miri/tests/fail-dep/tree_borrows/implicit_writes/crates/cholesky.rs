// Causes problems in: `adskalman`.
//@compile-flags: -Zmiri-tree-borrows -Zmiri-tree-borrows-implicit-writes
//@error-in-other-file: /reborrow through .* is forbidden/

fn main() {
    let m = nalgebra::Matrix2::<f64>::identity();
    let _chol = nalgebra::linalg::Cholesky::new(m);
}
