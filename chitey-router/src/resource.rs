use std::{error::Error, future::Future};
pub type Responder = Result<(http::response::Builder, bytes::Bytes), Box<dyn Error>>;

use chitey_server::guard::Guard;
use urlpattern::{UrlPattern, UrlPatternInit};

// type Task<T: Future<Output = Responder> + Send, U> = fn(U) -> T;
type Task<T, U> = fn(U) -> T;

pub struct Resource<T: Future<Output = Responder> + Send, U> {
    rdef: UrlPattern,
    name: Option<String>,
    register: Vec<Task<T, U>>,
    guard: Guard,
}

impl<T, U> Resource<T, U>
where
    T: Future<Output = Responder> + Send,
{
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
            register: vec![],
            guard: Guard::Get,
        }
    }
    pub fn regist(mut self, handler: Task<T, U>) -> Self {
        self.register.push(handler);
        self
    }
    pub fn name(mut self, nm: &str) -> Self {
        self.name = Some(nm.to_string());
        self
    }
    pub fn guard(mut self, g: Guard) -> Self {
        self.guard = g;
        self
    }
}
