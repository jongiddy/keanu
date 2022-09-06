use std::convert::Infallible;
use std::net::SocketAddr;
use async_stream::stream;
use hyper::body::Bytes;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

const SIZE: usize = 1 << 38;
const CHUNK_SIZE: usize = 1 << 20;

static ZEROS: [u8; CHUNK_SIZE] = [0; CHUNK_SIZE];


async fn handle_client(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let s = stream! {
        let mem = Bytes::from(&ZEROS[..]);
        for _ in 0..SIZE / CHUNK_SIZE {
            yield Ok::<_, Infallible>(mem.clone());
        }
    };
    let body = Body::wrap_stream(s);
    let response = Response::builder().header("Content-Length", SIZE.to_string()).body(body).unwrap();
    Ok(response)
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 6799));

    let make_service = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle_client))
    });

    // Then bind and serve...
    let server = Server::bind(&addr).serve(make_service);

    // And run forever...
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
