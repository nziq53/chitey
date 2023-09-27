use std::{
    io,
    net::{IpAddr, Ipv4Addr, SocketAddr, ToSocketAddrs},
    path::PathBuf, error::Error,
};

use crate::{server::{util::{get_certs_and_key, process_result, CustomOption}, http_server::{launch_http_server, HttpServerOpt}, https_server::{launch_https_server, HttpsServerOpt}, http3_server::{launch_http3_server, Http3ServerOpt}}, process::save_pid};

#[derive(Clone)]
pub struct Certs {
    pub cert: PathBuf,
    pub key: PathBuf,
}

pub trait HttpServiceFactory {
    fn register(&self);
}

pub struct WebServer {
    cert: Option<Certs>,
    listen: SocketAddr,
    tls_listen: SocketAddr,
    redirect: Option<String>,
    factories: Vec<Box<dyn HttpServiceFactory + 'static>>,
}

impl WebServer {
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
        F: HttpServiceFactory + 'static,
    {
        self.factories.push(Box::new(factory));
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

    pub fn redirect<T: std::ops::Deref<Target=String>>(mut self, url: T) -> Self {
        self.redirect = Some(url.as_str().to_string());
        self
    }

    pub fn tls(mut self, cert: Certs) -> Self {
        self.cert = Some(cert);
        self
    }

    pub async fn run(self) -> Result<(), Box<dyn Error>> {
        for mut factory in self.factories {
            let factory = factory.as_mut();
            factory.register();
        }

        if let Some(cert) = self.cert {
            let tls_certs_key = get_certs_and_key(cert)?;
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
                process_result(launch_https_server(tls_certs_key.clone(), https_server_opt.clone()).await);
            }
            });
            let handle_http3 = tokio::spawn(async move {
            loop {
                process_result(launch_http3_server(tls_certs_key2.clone(), http3_server_opt.clone()).await);
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
