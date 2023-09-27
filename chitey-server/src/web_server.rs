use async_trait::async_trait;
use futures_util::lock::Mutex;
use urlpattern::{UrlPatternInit, UrlPatternMatchInput};
use std::{
    io,
    net::{IpAddr, Ipv4Addr, SocketAddr, ToSocketAddrs},
    path::PathBuf, error::Error, pin::Pin, sync::{RwLock, Arc},
};

use hyper::Body;

use crate::{server::{util::{get_certs_and_key, process_result}, http_server::{launch_http_server, HttpServerOpt}, https_server::{launch_https_server, HttpsServerOpt}, http3_server::{launch_http3_server, Http3ServerOpt}}, process::save_pid, resource::{Resource, Responder}, tuple::{Tuple, Path}};

#[derive(Clone)]
pub struct Factories {
    pub(crate) factories: Vec<(Resource, Arc<Mutex<Box<dyn HttpServiceFactory + 'static + Send + Sync>>>)>,
}
unsafe impl Send for Factories {}
unsafe impl Sync for Factories {}

#[derive(Clone)]
pub struct Certs {
    pub cert: PathBuf,
    pub key: PathBuf,
}

#[async_trait(?Send)]
pub trait HttpServiceFactory: Sync
{
    fn register(&self) -> Resource;
    async fn handler_func(self, url: UrlPatternMatchInput, req: Request) -> Responder;
}

pub struct WebServer {
    cert: Option<Certs>,
    listen: SocketAddr,
    tls_listen: SocketAddr,
    redirect: Option<String>,
    factories: Vec<(Resource, Box<dyn HttpServiceFactory + 'static + Send + Sync>)>,
}

impl WebServer
{
    pub fn new() -> Self {
        let localhost = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        Self {
            cert: None,
            listen: SocketAddr::new(localhost, 8080),
            tls_listen: SocketAddr::new(localhost, 8443),
            redirect: None,
            factories: Vec::new(),
        }
    }

    pub fn service<F>(mut self, factory: F) -> Self
    where
        F: HttpServiceFactory + 'static + Send + Sync,
    {
        let resource = factory.register();
        self.factories.push((resource, Box::new(factory)));
        self
    }

    pub fn bind<A>(mut self, address: A) -> io::Result<Self>
    where
        A: ToSocketAddrs,
    {
        match address.to_socket_addrs() {
            Ok(v) => {
                for addr in v.collect::<Vec<SocketAddr>>() {
                    self.listen = addr;
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
                    self.tls_listen = addr;
                }
            }
            Err(e) => return Err(e),
        };

        Ok(self)
    }

    pub fn redirect<U: std::ops::Deref<Target=String>>(mut self, url: U) -> Self {
        self.redirect = Some(url.as_str().to_string());
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
            factories.push((res.clone(), fac.clone()));
            factories2.push((res, fac.clone()));
        }
        let factories = Factories{ factories };
        let factories2 = Factories{ factories: factories2 };

        if let Some(cert) = self.cert {
            let tls_certs_key = match get_certs_and_key(cert) {
                Ok(v) => v,
                Err(_) => return Err(ChiteyError::KeyAnalyzeError),
            };
            let tls_certs_key2 = tls_certs_key.clone();
            let http_server_opt = HttpServerOpt{ listen: self.listen, redirect: self.redirect };
            let https_server_opt = HttpsServerOpt{listen: self.tls_listen};
            let http3_server_opt = Http3ServerOpt{listen: self.tls_listen};

            let handle_http = tokio::spawn(async move {
            loop {
                process_result(launch_http_server(http_server_opt.clone(), save_pid).await);
            };
            });
            let handle_https = tokio::spawn(async move {
                loop {
                    process_result(launch_https_server(tls_certs_key.clone(), https_server_opt.clone(), Arc::new(RwLock::new(factories.clone()))).await);
                }
            });
            let handle_http3 = tokio::spawn(async move {
            loop {
                process_result(launch_http3_server(tls_certs_key2.clone(), http3_server_opt.clone(), factories2.clone()).await);
            }
            });
            let (_, _, _) = tokio::join!(
            handle_http,
            handle_https,
            handle_http3,
            );
        };

        eprintln!("You must set key always!! Right or Fake or not!!");
      
        Ok(())
    }
}

pub type Request = (http::Request<()>, bool);


use thiserror::Error;
#[derive(Error, Debug)]
pub enum ChiteyError {
    #[error("extract value failed")]
    UrlPatternError,
    #[error("server failed")]
    InternalServerError,
    #[error("cannot analyze key")]
    KeyAnalyzeError,
}

// async fn handler_func(url: UrlPatternMatchInput, req: Request) -> Responder {
//     let res: Resource;
//     let pattern = res.get_rdef();
//     // if let Ok(Some(result)) = pattern.exec(url) {
//     //     println!("{}",result.pathname.groups.get("id").unwrap().to_string());
//     //     // return #name().await;
//     // };
    
//     let url_ptn_result = match pattern.exec(url)? {
//         Some(v) => v,
//         None => return Err(Box::new(ChiteyError::UrlPatternError)),
//     };
//     let o = match url_ptn_result.pathname.groups.get("") {
//         Some(v) => v,
//         None => return Err(Box::new(ChiteyError::UrlPatternError)),
//     };
//     return Err(Box::new(ChiteyError::UrlPatternError));
// }
