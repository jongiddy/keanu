use std::io::Write;
use std::os::unix::net::{UnixListener, UnixStream};
use std::thread;

const SIZE: usize = 1 << 38;
const CHUNK_SIZE: usize = 1 << 20;

static ZEROS: [u8; CHUNK_SIZE] = [0; CHUNK_SIZE];

fn handle_client(mut stream: UnixStream) {
    for _ in 0..SIZE / CHUNK_SIZE {
        stream.write_all(&ZEROS).unwrap();
    }
}

fn main() {
    _ = std::fs::remove_file("/tmp/keanu.sock");
    let listener = UnixListener::bind("/tmp/keanu.sock").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }
}
