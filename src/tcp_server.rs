use std::io::Write;
use std::net::{TcpStream, TcpListener};
use std::thread;

const SIZE: usize = 1 << 38;
const CHUNK_SIZE: usize = 1 << 20;

static ZEROS: [u8; CHUNK_SIZE] = [0; CHUNK_SIZE];

fn handle_client(mut stream: TcpStream) {
    for _ in 0..SIZE / CHUNK_SIZE {
        stream.write_all(&ZEROS).unwrap();
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6798").unwrap();

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
