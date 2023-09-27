# メモ
## マクロ適用後
cargo install cargo-expand
cargo expand

```rust
fn main() {
    let body = async {
        {
            ::std::io::_print(format_args!("Hello, world!\n"));
        };
    };
    #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
#[allow(non_camel_case_types, missing_docs)]
pub struct greet;
impl ::chitey::HttpServiceFactory for greet {
    fn register(&self) -> ::chitey::Resource {
        async fn greet(name: String) -> Responder {
            {
                ::std::io::_print(format_args!("Hello {0}!\n", name));
            };
            let builder = Response::builder();
            let ret = Bytes::copy_from_slice(b"source");
            Ok((builder, ret))
        }
        let __resource = ::chitey::Resource::new("/hello/{name}").regist(greet);
        return __resource;
    }
}
#[allow(non_camel_case_types, missing_docs)]
pub struct doubb;
impl ::chitey::HttpServiceFactory for doubb {
    fn register(&self) -> ::chitey::Resource {
        async fn doubb((name, id): (u32, String)) -> Responder {
            {
                let res = ::alloc::fmt::format(
                    format_args!("Hello {0}! id:{1}", name, id),
                );
                res
            };
            let builder = Response::builder();
            let ret = Bytes::copy_from_slice(b"source");
            Ok((builder, ret))
        }
        let __resource = ::chitey::Resource::new("/{id}/{name}").regist(doubb);
        return __resource;
    }
}
```

```rust
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use actix_web::{get, web, App, HttpServer, Responder};
#[allow(non_camel_case_types, missing_docs)]
pub struct greet;
impl ::actix_web::dev::HttpServiceFactory for greet {
    fn register(self, __config: &mut actix_web::dev::AppService) {
        async fn greet(req: web::Path<(u32, String)>) -> impl Responder {
            let (name, id) = req.to_owned();
            {
                let res = ::alloc::fmt::format(
                    format_args!("Hello {0}! id:{1}", name, id),
                );
                res
            }
        }
        let __resource = ::actix_web::Resource::new("/{id}/{name}")
            .name("greet")
            .guard(::actix_web::guard::Get())
            .to(greet);
        ::actix_web::dev::HttpServiceFactory::register(__resource, __config);
    }
}
fn main() -> std::io::Result<()> {
    <::actix_web::rt::System>::new()
        .block_on(async move {
            {
                HttpServer::new(|| { App::new().service(greet) })
                    .bind("127.0.0.1:8080")?
                    .run()
                    .await
            }
        })
}
```