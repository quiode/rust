// Causes problems in: `a2lfile`, `acir`, `actix-files`.
//@compile-flags: -Zmiri-tree-borrows -Zmiri-tree-borrows-implicit-writes -Zmiri-disable-isolation
//@error-in-other-file: /write access through .* is forbidden/

fn main() {
    use std::io::{Read, Seek, SeekFrom, Write};

    let mut tmpfile = tempfile::tempfile().unwrap();
    tmpfile.write_all(b"test content").unwrap();
    tmpfile.seek(SeekFrom::Start(0)).unwrap();

    let bufsize = b"test content".len();
    let mut buffer = Vec::with_capacity(bufsize);
    let read_result = tmpfile.read_to_end(&mut buffer);

    assert!(read_result.is_ok());
    assert_eq!(buffer, b"test content");
}
