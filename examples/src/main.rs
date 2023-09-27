use chitey::{get, Responder, WebServer, post, Certs};
use bytes::Bytes;
use http::Response;

#[get("/hello/:name")]
async fn greet((name,): (String,)) -> Responder {
    println!("Hello {name}!");

    let builder = Response::builder();
    let ret = Bytes::copy_from_slice(b"source");
    Ok((builder, ret))
}

#[get("/:id/:name")]
async fn doubb((id, name): (u32, String)) -> Responder {
    format!("Hello {}! id:{}", name, id);

    let builder = Response::builder();
    let ret = Bytes::copy_from_slice(b"source");
    Ok((builder, ret))
}

#[post("/:id/:name")]
async fn dd((id, name): (u32, String)) -> Responder {
    format!("Hello {}! id:{}", name, id);

    let builder = Response::builder();
    let ret = Bytes::copy_from_slice(b"source");
    Ok((builder, ret))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    WebServer::new()
    .bind("localhost:18080").unwrap()
    .tls_bind("localhost:18443").unwrap()
    .tls(Certs { cert: "server.cert".into(), key: "server.key".into() })
    .service(greet)
    .service(doubb)
    .run()
    .await
}
