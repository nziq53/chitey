use chitey::{get, Responder, WebServer, post, Certs, Request, ChiteyError, http::Response, Bytes, throw_chitey_internal_server_error as throw};

#[get("/:id/:name")]
async fn greet((id, name): (u32, String), _req: Request) -> Responder {
    throw(Response::builder().body(Bytes::from(format!("Hello {}! id:{}", name, id)).into()))
}


#[get("/:id/:name")]
async fn greet_not_number((id, name): (String, String), _req: Request) -> Responder {
    throw(Response::builder().body(Bytes::from(format!("HelloNotNumberId {}! id:{}", name, id)).into()))
}

#[get("/")]
async fn home((): (), _req: Request) -> Responder {
    throw(Response::builder().body(Bytes::copy_from_slice(b"source").into()))
}

#[get("/all/:all")]
async fn all_all_path((): (), _req: Request) -> Responder {
    throw(Response::builder().body(Bytes::copy_from_slice(b"source").into()))
}

#[get("/:all")]
async fn all_path((all,): (String,), _req: Request) -> Responder {
    throw(Response::builder().body(Bytes::from(format!("All String: {}", all)).into()))
}

#[get("/**")]
async fn notfound((): (), _req: Request) -> Responder {
    throw(Response::builder().body(Bytes::copy_from_slice(b"404 not found").into()))
}

#[post("/**")]
async fn notfoundpost((): (), _req: Request) -> Responder {
    throw(Response::builder().body(Bytes::copy_from_slice(b"404 not found").into()))
}

#[tokio::main]
async fn main() -> Result<(), ChiteyError> {
    println!("Hello, world!");
    WebServer::new()
    .bind("localhost:18081").unwrap()
    .tls_bind("localhost:18443").unwrap()
    .tls(Certs { cert: "server.cert".into(), key: "server.key".into() })
    .service(greet)
    .service(greet_not_number)
    .service(home)
    .service(all_all_path)
    .service(all_path)
    .service(notfound)
    .service(notfoundpost)
    .run()
    .await
}
