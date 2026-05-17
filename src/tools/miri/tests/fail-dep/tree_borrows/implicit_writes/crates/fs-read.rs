// Causes problems in: `a2lfile`.
//@compile-flags: -Zmiri-tree-borrows -Zmiri-tree-borrows-implicit-writes -Zmiri-disable-isolation
//@error-in-other-file: /write access through .* is forbidden/

fn main() {
    use std::io::Write;

    let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
    tmpfile.write_all(b"test content").unwrap();
    let path = tmpfile.path();

    let file_text = String::from_utf8(std::fs::read(path).unwrap()).unwrap();

    assert_eq!(file_text, "test content");
}
