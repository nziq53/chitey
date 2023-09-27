use chitey::{get, Responder};
use bytes::Bytes;
use http::Response;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

}

#[get("/hello/:name")]
async fn greet(name: String) -> Responder {
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
// use chitey::HttpServiceFactory;
// use chitey::Resource;

// #[tokio::main]
// async fn main() {
//     println!("Hello, world!");
// }
// #[allow(non_camel_case_types, missing_docs)]
// pub struct greet;
// impl HttpServiceFactory for greet {
//     fn register(&self) {
//         async fn greet(name: Path<String>) -> Responder {
//             {
//                 println!("Hello {0}!\n", name);
//             };
//             let builder = Response::builder();
//             let ret = Bytes::copy_from_slice(b"source");
//             Ok((builder, ret))
//         }
//         let mut __resource = Resource::new("/hello/{name}");
//             __resource.regist(greet);
//     }
// }
// #[allow(non_camel_case_types, missing_docs)]
// pub struct doubb;
// impl HttpServiceFactory for doubb {
//     fn register(&self) {
//         async fn doubb(req: Path<(u32, String)>) -> Responder {
//             let (id, name) = req.to_owned();
//             {
//                 format!("Hello {}! id:{}", name, id);
//             };
//             let builder = Response::builder();
//             let ret = Bytes::copy_from_slice(b"source");
//             Ok((builder, ret))
//         }
//         let mut __resource = Resource::new("/{id}/{name}")
//             .regist(doubb);
//     }
// }