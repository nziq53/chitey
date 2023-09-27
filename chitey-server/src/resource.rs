use std::{error::Error, future::Future, pin::Pin};
pub type Responder = Result<(http::response::Builder, bytes::Bytes), Box<dyn Error>>;

use http::Request;
use hyper::Body;
use urlpattern::{UrlPattern, UrlPatternInit};

use crate::{guard::Guard, tuple::{self, TupleAppend, Tuple, TupleWrapper}};

// type Task<T: Future<Output = Responder> + Send, U> = fn(U) -> T;
// type Task<T, U> = fn(U) -> T;

pub struct Resource {
    rdef: UrlPattern,
    name: Option<String>,
    register: Option<BoxFuture<Responder>>,
    guard: Guard,
}

// pub struct BoxedResource {
//     resource: Pin<Box<Resource<T, U>>>,
// }

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

    pub fn regist<F, Req, Fut, Tp>(mut self, handler: F) -> Self
    where
        F: Fn(Req) -> Fut + Clone,
        Req: Tuple + From<TupleWrapper<(String,)>>,
        Tp: Tuple,
        Fut: Future<Output = Responder>,
    {
        let register_wrap = Box::pin(|req: Request<Body>, isHttp3: bool| async move {
            let tuple = ();
            let tuple = tuple.append("#".to_owned());
            let tuple2 = TupleWrapper::new(tuple);
            // let y: Req = tuple.into();
            handler(tuple2.into()).await
        });
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
