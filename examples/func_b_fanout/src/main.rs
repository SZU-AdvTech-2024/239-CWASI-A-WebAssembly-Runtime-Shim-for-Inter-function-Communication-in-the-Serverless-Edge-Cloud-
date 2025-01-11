use std::net::SocketAddr;

use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response};
use tokio::net::TcpListener;
use chrono::{SecondsFormat};

async fn echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {

        (&Method::GET, "/") => Ok(Response::new(Body::from(
            "👋 Hello World 🌍",
        ))),


        (&Method::POST, "/hello") => {
            let name = hyper::body::to_bytes(req.into_body()).await?;
            let name_string = String::from_utf8(name.to_vec()).unwrap();
            let start= chrono::offset::Utc::now().to_rfc3339_opts(SecondsFormat::Nanos, true);
            let received = format!("Received at {:?} \n {} len",start,name_string);
            //let answer = format!("{}{}", "Hello ".to_owned(), name_string);
            println!("Received {} len {}",start,name.len());
            Ok(Response::new(Body::from(received)))
        }

        _ => {
            Ok(Response::new(Body::from("😡 try again")))
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main2() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 1234));

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            if let Err(err) = Http::new().serve_connection(stream, service_fn(echo)).await {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}

#[no_mangle]
pub extern fn allocate(size: usize) -> i32 {
    // Grow the WebAssembly memory by the requested size and return the pointer
    let ptr = wasm_alloc(size);
    ptr as i32  // Return the pointer (as an offset within WebAssembly memory)
}

fn wasm_alloc(size: usize) -> *mut u8 {
    // Simulate memory allocation by creating a buffer with the requested size
    let mut buffer = Vec::with_capacity(size);
    let ptr = buffer.as_mut_ptr();
    std::mem::forget(buffer);  // Don't drop the buffer, just leak it
    ptr
}
fn main() {
    println!("Greetings from func_a!");
    // cwasi_function();
}

#[no_mangle]
pub extern fn cwasi_function(ptr: i32, len: i32) {
    main2();
}
