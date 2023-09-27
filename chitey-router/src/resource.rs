use std::future::Future;
pub type Responder = Result<(http::response::Builder, bytes::Bytes), Error>;

use crate::handler::Error;
use urlpattern::{UrlPattern, UrlPatternInit};

// type Task<T: Future<Output = Responder> + Send, U> = fn(U) -> T;
type Task<T, U> = fn(U) -> T;

pub struct Resource<T: Future<Output = Responder> + Send, U>
// where
// F: Handler,
{
    rdef: UrlPattern,
    name: Option<String>,
    register: Vec<Task<T, U>>,
}

impl<T, U> Resource<T, U>
where
    T: Future<Output = Responder> + Send,
    // where
    // F: Handler,
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
        }
    }
    // pub fn regist<T: HttpServiceFactory + 'static>(mut self, register: T) {
    //     self.register.push(Box::new(register));
    // }
    pub fn regist(mut self, handler: Task<T, U>) {
        self.register.push(handler);
    }
}

pub trait HttpServiceFactory {
    fn register(&self);
}

// pub struct HandleRegister {
//     handler: Vec<Box<i128>>,
// }

// impl HandleRegister {
//     pub fn regist<T>(&mut self) {
//         let my_closure = |req: &Request<T>, isHttp3: bool| async {
//             // 関数の中身
//             let hh: Responder;
//             hh
//         };
//         let boxed_closure = Box::new(my_closure);
//     }
// }
