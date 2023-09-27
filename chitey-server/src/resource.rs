use std::{error::Error, future::Future, pin::Pin};
pub type Responder = Result<(http::response::Builder, bytes::Bytes), Box<dyn Error>>;

use http::Request;
use hyper::Body;
use urlpattern::{UrlPattern, UrlPatternInit};

use crate::{guard::Guard, tuple::{TupleAppend, Tuple, Path}, fn_service};

// type Task<T: Future<Output = Responder> + Send, U> = fn(U) -> T;
// type Task<T, U> = fn(U) -> T;

pub struct Resource {
    rdef: UrlPattern,
    name: Option<String>,
    register: Option<BoxedHandleFunc>,
    guard: Guard,
}

type BoxedHandleFunc = Pin<Box<dyn Fn(Request<Body>, bool) -> dyn Future<Output = Responder>>>;

// pub struct BoxedResource {
//     resource: Pin<Box<Resource<T, U>>>,
// }
use crate::fn_service::fn_service;

impl Resource {
    /// Constructs new resource that matches a `path` pattern.
    pub fn new(path: &str) -> Self {
        let path = <UrlPattern>::parse(UrlPatternInit {
            pathname: Some(path.to_owned()),
            ..Default::default()
        })
        .unwrap();

        Resource {
            rdef: path,
            name: None,
            register: None,
            guard: Guard::Get,
        }
    }

    pub fn regist<F, Fut, T>(self, handler: F) -> Self
    where
        F: Fn(Path<T>, (Request<Body>, bool)) -> Fut + Clone,
        Fut: Future<Output = Responder>,
    {
        // let input = UrlPatternMatchInput::Url(url);
        // let tuple2 = Path::new(tuple);
        // if (self.rdef.exec())
        let _register_wrap = fn_service(move |req: (Request<Body>, bool)| {
            let handler = handler.clone();
            let (req, is_http3) = req;

            let tuple: Path<T> = Path::tuple_new(self.rdef, req.uri().clone());
            async move {
                handler(tuple, (req, is_http3)).await
                // let tuple = ();
                // let tuple = tuple.append("#".to_owned());
                // let y: Req = tuple2.into();
                // let pa = Path::new(tuple);
            }
        });
        // self.register = Some(_register_wrap);
        self
    }

    // pub fn regist(mut self, handler: BoxFuture<Responder>) -> Self {
    //     self.register = Some(Box::pin(handler));
    //     self
    // }
    pub fn name(mut self, nm: &str) -> Self {
        self.name = Some(nm.to_string());
        self
    }
    pub fn guard(mut self, g: Guard) -> Self {
        self.guard = g;
        self
    }
}

pub trait FromRequest: Sized {
    type Error: Into<Box<dyn Error>>;
    type Future: Future<Output = Result<Self, Self::Error>>;
}

pub trait Handler<Args>: Clone + 'static {
    type Future: Future<Output = Responder>;

    fn call(&self, args: Args) -> Self::Future;
}

pub type BoxFuture<T> = Pin<Box<dyn Future<Output = T>>>;
