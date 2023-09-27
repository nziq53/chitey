use std::{error::Error, future::Future, pin::Pin};
pub type Responder = Result<(http::response::Builder, bytes::Bytes), ChiteyError>;

use http::Request;
use hyper::Body;
use urlpattern::{UrlPattern, UrlPatternInit};

use crate::{guard::Guard, tuple::{TupleAppend, Tuple, Path}, fn_service, web_server::{self, ChiteyError}};

#[derive(Debug)]
pub struct Resource {
    rdef: UrlPattern,
    url_ptn: String,
    name: Option<String>,
    guard: Guard,
}

impl Resource {
    /// Constructs new resource that matches a `path` pattern.
    pub fn new(path: &str) -> Self {
        let ptn = <UrlPattern>::parse(UrlPatternInit {
            pathname: Some(path.to_owned()),
            ..Default::default()
        })
        .unwrap();

        Resource {
            rdef: ptn,
            url_ptn: path.to_string(),
            name: None,
            guard: Guard::Get,
        }
    }
    pub fn name(mut self, nm: &str) -> Self {
        self.name = Some(nm.to_string());
        self
    }
    pub fn guard(mut self, g: Guard) -> Self {
        self.guard = g;
        self
    }
    pub fn get_rdef(self) -> UrlPattern {
        self.rdef
    }
}

impl Clone for Resource {
    fn clone(&self) -> Self {
        Self { rdef: <UrlPattern>::parse(UrlPatternInit {
            pathname: Some(self.url_ptn.clone().to_owned()),
            ..Default::default()
        })
        .unwrap(), url_ptn: self.url_ptn.clone(), name: self.name.clone(), guard: self.guard.clone() }
    }
}
