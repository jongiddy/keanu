use std::io::Read;
use std::os::unix::net::UnixStream;
use std::time::Instant;

const SIZE: usize = 1 << 38;

const MAX: usize = 20;

fn main() {
    let mut buf: [u8; 1 << MAX] = [0; 1 << MAX];
    for chunk_exp in 10..=MAX {
        let chunk_size = 1 << chunk_exp;
        let mut bytes_read = 0usize;
        let mut stream = UnixStream::connect("/tmp/keanu.sock").unwrap();
        let start = Instant::now();
        loop {
            match stream.read(&mut buf[..chunk_size]).unwrap() {
                0 => {
                    let elapsed = start.elapsed();
                    let mbps = bytes_read as f64 / (1 << 20) as f64 / elapsed.as_secs_f64();
                    if bytes_read != SIZE {
                        println!("Expected {SIZE}, got {bytes_read}");
                    }
                    println!("{chunk_size} bytes: {bytes_read} bytes read in {elapsed:?} seconds - {mbps} MiB/s");
                    break;
                }
                n => {
                    bytes_read += n;
                }
            }
        }
    }
}
