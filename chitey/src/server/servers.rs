use std::{path::PathBuf, fs};

use rustls::{Certificate, PrivateKey};
use tracing::info;

use crate::process::save_pid;

use super::{http_server::{launch_http_server, HttpServerOpt}, https_server::{launch_https_server, HttpsServerOpt}, http3_server::{Http3ServerOpt, launch_http3_server}};

#[derive(Clone)]
pub struct TlsCertsKey {
  pub certs: Vec<Certificate>,
  pub key: PrivateKey,
}

pub fn error(err: String) -> std::io::Error {
  std::io::Error::new(std::io::ErrorKind::Other, err)
}

pub struct Certs {
  pub cert: PathBuf,
  pub key: PathBuf,
}

pub async fn server_build (
  certs: Certs,
  http_server_opt: HttpServerOpt,
  https_server_opt: HttpsServerOpt,
  http3_server_opt: Http3ServerOpt,
) -> Result<(), Box<dyn std::error::Error>> {
  // create quinn server endpoint and bind UDP socket

  let tls_certs_key = get_certs_and_key(certs)?;
  let tls_certs_key2 = tls_certs_key.clone();

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

  Ok(())
}

fn get_certs_and_key(certs: Certs) -> Result<TlsCertsKey, Box<dyn std::error::Error>> {
  let Certs { cert, key } = certs;

  if cert.extension().unwrap() == "pem" {
    info!("cert file is pem file");
    Ok( TlsCertsKey{certs: load_certs(cert)?, key: load_private_key(key)?})
  } else {
    Ok( TlsCertsKey{certs: vec![Certificate(std::fs::read(cert)?)], key: PrivateKey(std::fs::read(key)?)})
  }
}


fn load_certs(filepath: PathBuf) -> std::io::Result<Vec<rustls::Certificate>> {
  // Open certificate file.
  let certfile = fs::File::open(filepath.clone())
      .map_err(|e| error(format!("failed to open {}: {}", filepath.to_string_lossy(), e)))?;
  let mut reader = std::io::BufReader::new(certfile);

  // Load and return certificate.
  let certs = rustls_pemfile::certs(&mut reader)
      .map_err(|_| error("failed to load certificate".into()))?;
  Ok(certs
      .into_iter()
      .map(rustls::Certificate)
      .collect())
}

fn load_private_key(filepath: PathBuf) -> std::io::Result<rustls::PrivateKey> {
  // Open keyfile.
  let keyfile = fs::File::open(filepath.clone())
      .map_err(|e| error(format!("failed to open {}: {}", filepath.to_string_lossy(), e)))?;
  let mut reader = std::io::BufReader::new(keyfile);

  // Load and return a single private key.
  let keys = rustls_pemfile::pkcs8_private_keys(&mut reader)
      .map_err(|_| error("failed to load private key".into()))?;
  if keys.len() != 1 {
      return Err(error("expected a single private key".into()));
  }
  Ok(rustls::PrivateKey(keys[0].clone()))
}

fn process_result<T, R: std::fmt::Display>(result: Result<T, R>) -> Option<T> {
  match result {
      Ok(value) => {
        Some(value)
      }
      Err(error) => {
        println!("Error: {}", error);
        None
      }
  }
}