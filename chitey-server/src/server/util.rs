use std::{fs, path::PathBuf};

use rustls::{Certificate, PrivateKey};
// use tracing::info;

use crate::web_server::{Certs, ChiteyError};

#[derive(Clone)]
pub struct TlsCertsKey {
    pub certs: Vec<Certificate>,
    pub key: PrivateKey,
}

pub fn get_certs_and_key(certs: Certs) -> Result<TlsCertsKey, Box<dyn std::error::Error>> {
    let Certs { cert, key } = certs;

    if cert.extension().unwrap() == "pem" {
        // info!("cert file is pem file");
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
        ChiteyError::InternalServerError(format!(
            "failed to open {}: {}",
            filepath.to_string_lossy(),
            e
        ))
    }).unwrap();
    let mut reader = std::io::BufReader::new(certfile);

    // Load and return certificate.
    let certs = rustls_pemfile::certs(&mut reader)
        .map_err(|_| ChiteyError::InternalServerError("failed to load certificate".into())).unwrap();
    Ok(certs.into_iter().map(rustls::Certificate).collect())
}

pub fn load_private_key(filepath: PathBuf) -> std::io::Result<rustls::PrivateKey> {
    // Open keyfile.
    let keyfile = fs::File::open(filepath.clone()).map_err(|e| {
        ChiteyError::InternalServerError(format!(
            "failed to open {}: {}",
            filepath.to_string_lossy(),
            e
        ))
    }).unwrap();
    let mut reader = std::io::BufReader::new(keyfile);

    // Load and return a single private key.
    let keys = rustls_pemfile::pkcs8_private_keys(&mut reader)
        .map_err(|_| ChiteyError::InternalServerError("failed to load private key".into())).unwrap();
    if keys.len() != 1 {
        return Err(ChiteyError::InternalServerError("expected a single private key".into())).unwrap();
    }
    Ok(rustls::PrivateKey(keys[0].clone()))
}

#[inline]
pub fn process_result<T, R: std::fmt::Display>(result: Result<T, R>) -> Option<T> {
    match result {
        Ok(value) => Some(value),
        Err(error) => {
            tracing::error!("Error: {}", error);
            None
        }
    }
}

#[inline]
pub fn throw_chitey_internal_server_error<T, E>(res: Result<T, E>) -> Result<T, ChiteyError>
where
    E: std::error::Error,
{
    match res {
        Ok(v) => Ok(v),
        Err(e) =>Err(ChiteyError::InternalServerError(e.to_string())),
    }
}

#[inline]
pub fn cors_builder() -> http::response::Builder {
    let builder = hyper::Response::builder();
    let builder = builder.header("Access-Control-Allow-Origin", "*");
    builder
}
