use std::{fs, path::PathBuf};

use rustls::{Certificate, PrivateKey};
use tracing::info;

use crate::{process::save_pid, web_server::Certs};

use super::{
    http3_server::{launch_http3_server, Http3ServerOpt},
    http_server::{launch_http_server, HttpServerOpt},
    https_server::{launch_https_server, HttpsServerOpt},
};

#[derive(Clone)]
pub struct TlsCertsKey {
    pub certs: Vec<Certificate>,
    pub key: PrivateKey,
}

pub fn error(err: String) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, err)
}

pub fn get_certs_and_key(certs: Certs) -> Result<TlsCertsKey, Box<dyn std::error::Error>> {
    let Certs { cert, key } = certs;

    if cert.extension().unwrap() == "pem" {
        info!("cert file is pem file");
        Ok(TlsCertsKey {
            certs: load_certs(cert)?,
            key: load_private_key(key)?,
        })
    } else {
        Ok(TlsCertsKey {
            certs: vec![Certificate(std::fs::read(cert)?)],
            key: PrivateKey(std::fs::read(key)?),
        })
    }
}

pub fn load_certs(filepath: PathBuf) -> std::io::Result<Vec<rustls::Certificate>> {
    // Open certificate file.
    let certfile = fs::File::open(filepath.clone()).map_err(|e| {
        error(format!(
            "failed to open {}: {}",
            filepath.to_string_lossy(),
            e
        ))
    })?;
    let mut reader = std::io::BufReader::new(certfile);

    // Load and return certificate.
    let certs = rustls_pemfile::certs(&mut reader)
        .map_err(|_| error("failed to load certificate".into()))?;
    Ok(certs.into_iter().map(rustls::Certificate).collect())
}

pub fn load_private_key(filepath: PathBuf) -> std::io::Result<rustls::PrivateKey> {
    // Open keyfile.
    let keyfile = fs::File::open(filepath.clone()).map_err(|e| {
        error(format!(
            "failed to open {}: {}",
            filepath.to_string_lossy(),
            e
        ))
    })?;
    let mut reader = std::io::BufReader::new(keyfile);

    // Load and return a single private key.
    let keys = rustls_pemfile::pkcs8_private_keys(&mut reader)
        .map_err(|_| error("failed to load private key".into()))?;
    if keys.len() != 1 {
        return Err(error("expected a single private key".into()));
    }
    Ok(rustls::PrivateKey(keys[0].clone()))
}

pub fn process_result<T, R: std::fmt::Display>(result: Result<T, R>) -> Option<T> {
    match result {
        Ok(value) => Some(value),
        Err(error) => {
            println!("Error: {}", error);
            None
        }
    }
}

#[derive(Clone)]
pub enum CustomOption<T> {
    None,
    Some(T),
}
unsafe impl<T> Send for CustomOption<T> {}
unsafe impl<T> Sync for CustomOption<T> {}
