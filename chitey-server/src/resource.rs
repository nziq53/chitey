pub type Responder = Result<(http::response::Builder, bytes::Bytes), ChiteyError>;

use urlpattern::{UrlPattern, UrlPatternInit};

use crate::{guard::Guard, web_server::ChiteyError};

#[derive(Debug)]
pub struct Resource {
    pub(crate) rdef: UrlPattern,
    pub(crate) url_ptn: String,
    pub(crate) name: Option<String>,
    pub(crate) guard: Guard,
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
