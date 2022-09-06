use std::time::Instant;

use futures_util::stream::StreamExt;
use hyper::body::Buf;
use hyper::{Client, StatusCode};
use hyper_unix_connector::{UnixClient, Uri};

const SIZE: usize = 1 << 38;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let client = Client::builder().build::<_, hyper::Body>(UnixClient);

    let start = Instant::now();

    let res = client
        .get(Uri::new("/tmp/keanu.sock", "/").into())
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let bytes_read = res
        .into_body()
        .fold(0usize, |size, data| async move {
            size + data.unwrap().remaining()
        })
        .await;

    let elapsed = start.elapsed();
    let mbps = bytes_read as f64 / (1 << 20) as f64 / elapsed.as_secs_f64();
    if bytes_read != SIZE {
        println!("Expected {SIZE}, got {bytes_read}");
    }
    println!("{bytes_read} bytes read in {elapsed:?} seconds - {mbps} MiB/s");
}
