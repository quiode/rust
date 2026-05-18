// Demonstrates that `libc::recv` triggers a Tree Borrows violation under implicit writes.
//
// This causes problems in: `actix-files`, `actix-multipart`.
//
//@ignore-target: windows
//@compile-flags: -Zmiri-tree-borrows -Zmiri-tree-borrows-implicit-writes -Zmiri-disable-isolation
//@error-in-other-file: /write access through .* is forbidden/

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    let handle = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        stream.write_all(b"hi").unwrap();
    });

    let mut stream = TcpStream::connect(addr).unwrap();
    let mut buf = [0u8; 2];
    stream.read(&mut buf).unwrap();

    handle.join().unwrap();
}
