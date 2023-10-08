use async_trait::async_trait;
use fnv::FnvHashMap;
use tokio::sync::Mutex;
use urlpattern::UrlPatternMatchInput;
use core::fmt;
use std::{
    io,
    net::{SocketAddr, ToSocketAddrs},
    path::PathBuf, pin::Pin, sync::Arc, any::{Any, TypeId}, ops::Deref, fmt::Formatter,
};

use hyper::Body;

use crate::{server::{util::{get_certs_and_key, process_result}, http_server::{launch_http_server, HttpServerOpt}, https_server::{launch_https_server, HttpsServerOpt}, http3_server::{launch_http3_server, Http3ServerOpt}}, process::save_pid, resource::{Resource, Responder}};


#[derive(Default)]
pub struct Data (FnvHashMap<TypeId, Box<dyn Any + Sync + Send>>);

impl Deref for Data {
    type Target = FnvHashMap<TypeId, Box<dyn Any + Sync + Send>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Data {
    pub fn new(data: FnvHashMap<TypeId, Box<dyn Any + Sync + Send>>) -> Self {
        Self(data)
    }

    pub fn default() -> Self {
        Self(FnvHashMap::default())
    }

    pub fn insert<D>(&mut self, data: D)
    where
        D: Any + Send + Sync,
    {
        self.0.insert(TypeId::of::<D>(), Box::new(data));
    }
}

impl fmt::Debug for Data {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_tuple("Data").finish()
    }
}

pub type Result<T, E = ChiteyError> = std::result::Result<T, E>;

pub struct ContextImpl {
    pub(crate) data: Data,
}

impl ContextImpl {
    pub fn new() -> Self {
        ContextImpl { data: Data::new(FnvHashMap::default()) }
    }

    #[must_use]
    pub fn insert<D>(mut self, data: D) -> Self
    where
        D: Any + Send + Sync,
    {
        self.data.insert(data);
        self
    }
}

#[derive(Clone)]
pub struct Context {
    data: Arc<ContextImpl>,
}

impl Context {
    pub fn new(data: ContextImpl) -> Self {
        Self {
            data: Arc::new(data),
        }
    }

    pub fn get<D>(&self) -> &D
    where
        D: Any + Send + Sync,
    {
        self.data_opt::<D>().unwrap_or_else(|| panic!("Data `{}` does not exist.", std::any::type_name::<D>()))
    }

    pub fn data_opt<D>(&self) -> Option<&D>
    where
        D: Any + Send + Sync,
    {
        self.data.data.get(&TypeId::of::<D>()).and_then(|d| d.downcast_ref::<D>())
    }
}

#[derive(Clone)]
pub struct Factories {
    pub(crate) factories: Vec<(Arc<Resource>, Arc<Mutex<Pin<Box<dyn HttpServiceFactory + 'static + Send + Sync>>>>)>,
    pub(crate) contexts: Context,
}
unsafe impl Send for Factories {}
unsafe impl Sync for Factories {}

#[derive(Clone)]
pub struct Certs {
    pub cert: PathBuf,
    pub key: PathBuf,
}

#[async_trait]
pub trait HttpServiceFactory: Sync
{
    fn register(&self) -> Resource;
    fn analyze_types(&self, url: UrlPatternMatchInput) -> bool;
    async fn handler_func(&self, url: UrlPatternMatchInput, req: Request) -> Responder;
}

pub struct WebServer {
    cert: Option<Certs>,
    listen: Option<SocketAddr>,
    tls_listen: Option<SocketAddr>,
    redirect: Option<String>,
    factories: Vec<(Resource, Pin<Box<dyn HttpServiceFactory + 'static + Send + Sync>>)>,
    data: Data,
}

impl WebServer
{
    pub fn new() -> Self {
        Self {
            cert: None,
            listen: None,
            tls_listen: None,
            redirect: None,
            factories: Vec::new(),
            data: Data::default(),
        }
    }

    pub fn service<F>(mut self, factory: F) -> Self
    where
        F: HttpServiceFactory + 'static + Send + Sync,
    {
        let resource = factory.register();
        self.factories.push((resource, Box::pin(factory)));
        self
    }

    pub fn bind<A>(mut self, address: A) -> io::Result<Self>
    where
        A: ToSocketAddrs,
    {
        match address.to_socket_addrs() {
            Ok(v) => {
                for addr in v.collect::<Vec<SocketAddr>>() {
                    self.listen = Some(addr);
                }
            }
            Err(e) => return Err(e),
        };

        Ok(self)
    }

    pub fn tls_bind<A>(mut self, address: A) -> io::Result<Self>
    where
        A: ToSocketAddrs,
    {
        match address.to_socket_addrs() {
            Ok(v) => {
                for addr in v.collect::<Vec<SocketAddr>>() {
                    self.tls_listen = Some(addr);
                }
            }
            Err(e) => return Err(e),
        };

        Ok(self)
    }

    pub fn redirect<U: std::ops::Deref<Target=str>>(mut self, url: U) -> Self {
        self.redirect = Some(url.to_string());
        self
    }

    pub fn tls(mut self, cert: Certs) -> Self {
        self.cert = Some(cert);
        self
    }

    pub async fn run(self) -> Result<(), ChiteyError> {
        let mut factories = Vec::new();
        let mut factories2 = Vec::new();
        for factory in self.factories {
            let (res, fact) = factory;
            let fac = Arc::new(Mutex::new(fact));
            factories.push((Arc::new(res.clone()), fac.clone()));
            factories2.push((Arc::new(res), fac.clone()));
        }

        let contexts = Context::new(ContextImpl { data: self.data });
        let factories = Factories{ factories, contexts: contexts.clone() };
        let factories2 = Factories{ factories: factories2, contexts: contexts.clone() };

        if let Some(cert) = self.cert {
            let tls_certs_key = match get_certs_and_key(cert) {
                Ok(v) => v,
                Err(e) => return Err(ChiteyError::KeyAnalyzeError(e.to_string())),
            };
            let tls_certs_key2 = tls_certs_key.clone();
            let handle_http = async {
                let factories = factories.clone();
                if let Some(li) = self.listen {
                    let http_server_opt = HttpServerOpt{ listen: li, redirect: self.redirect };
                    let _ = tokio::spawn(async move {
                        loop {
                            process_result(launch_http_server(http_server_opt.clone(), save_pid, factories.clone()).await);
                        };
                    }).await;
                }
            };
            let factories = factories.clone();
            let handle_https = async {
                if let Some(li) = self.tls_listen {
                    let https_server_opt = HttpsServerOpt{listen: li};
                    let http3_server_opt = Http3ServerOpt{listen: li};
                    let handle_https = tokio::spawn(async move {
                        loop {
                            process_result(launch_https_server(tls_certs_key.clone(), https_server_opt.clone(), factories.clone()).await);
                        }
                    });
                    let handle_http3 = tokio::spawn(async move {
                        loop {
                            process_result(launch_http3_server(tls_certs_key2.clone(), http3_server_opt.clone(), factories2.clone()).await);
                        }
                    });
                    let (_, _) = tokio::join!(handle_https, handle_http3);
                }
            };

            let (_, _) = tokio::join!(
                handle_http,
                handle_https,
            );
        };

        eprintln!("You must set key always!! Right or Fake or not!!");

        Ok(())
    }

    #[must_use]
    pub fn data<D>(mut self, data: D) -> Self
    where
        D: Any + Send + Sync
    {
        self.data.insert(data);
        self
    }
}

pub type Request = (http::Request<Body>, bool, Context);

use thiserror::Error;
#[derive(Error, Debug)]
pub enum ChiteyError {
    #[error("extract value failed")]
    UrlPatternError,
    #[error("server failed: {0}")]
    InternalServerError(String),
    #[error("cannot analyze key: {0}")]
    KeyAnalyzeError(String),
    #[error("failed kill server: {0}")]
    ServerKillError(String),
}
