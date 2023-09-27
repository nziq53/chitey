use std::{sync::{self, Arc}, net::SocketAddr, convert::Infallible, pin::Pin, task::{Context, Poll}, io::{BufWriter, Write}, fs::{File, self}, collections::HashMap};

use crate::response::response::handle_request_get;

use super::util::{TlsCertsKey, CustomOption};
use bytes::{BytesMut, BufMut};
use futures_util::{ready, Future, TryStreamExt};
use http::{Request, Response, StatusCode, Method};
use hyper::{server::{conn::{AddrIncoming, AddrStream}, accept::Accept}, service::{make_service_fn, service_fn}, Server, Body};
use mime::Mime;
use rustls::ServerConfig;
use tokio::io::{AsyncRead, ReadBuf, self, AsyncWrite};
use urlpattern::{UrlPatternInit, UrlPattern, UrlPatternMatchInput};
use super::util::error;
use tracing::info;


// HTTP/2 TLS など  chromeなどのブラウザはこちらに最初にアクセスしてくる
// https://github.com/quic-go/quic-go/issues/3890
// https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Alt-Svc#browser_compatibility
// https://http3-explained.haxx.se/ja/h3/h3-altsvc

#[derive(Clone)]
pub struct HttpsServerOpt {
  pub listen: SocketAddr,
}

pub async fn launch_https_server (tls_cert_key: TlsCertsKey, https_server_opt: HttpsServerOpt) -> Result<(), Box<dyn std::error::Error>> {
  let TlsCertsKey{certs, key} = tls_cert_key;
  let HttpsServerOpt{listen} = https_server_opt;

  let tls_config = {
    // Do not use client certificate authentication.
    let mut cfg = rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certs, key).unwrap();        // Configure ALPN to accept HTTP/2, HTTP/1.1 in that order.

    // cfg.alpn_protocols = b"\x02h2\x08http/1.1";
    cfg.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];
    sync::Arc::new(cfg)
  };

  let incoming = AddrIncoming::bind(&listen)
    .map_err(|e| error(format!("Incoming failed: {:?}", e))).expect("error");
  let service = make_service_fn(|_| async { Ok::<_, Infallible>(service_fn(handle_https_service))});
  let https_server = Server::builder(HyperAcceptor::new(tls_config, incoming)).serve(service);

  // Prepare a long-running future stream to accept and serve clients.

  // handle incoming connections and requests

  println!("Starting to serve on https://{}.", listen);
  let _ = https_server.await?;

  Ok(())
}

pub struct HyperAcceptor {
  config: Arc<ServerConfig>,
  incoming: AddrIncoming,
}

impl HyperAcceptor {
  pub fn new(config: Arc<ServerConfig>, incoming: AddrIncoming) -> HyperAcceptor {
      HyperAcceptor { config, incoming }
  }
}

enum State {
  Handshaking(tokio_rustls::Accept<AddrStream>),
  Streaming(tokio_rustls::server::TlsStream<AddrStream>),
}

// tokio_rustls::server::TlsStream doesn't expose constructor methods,
// so we have to TlsAcceptor::accept and handshake to have access to it
// TlsStream implements AsyncRead/AsyncWrite handshaking tokio_rustls::Accept first
pub struct HyperStream {
  state: State,
}

impl HyperStream {
  fn new(stream: AddrStream, config: Arc<ServerConfig>) -> HyperStream {
      let accept = tokio_rustls::TlsAcceptor::from(config).accept(stream);
      HyperStream {
          state: State::Handshaking(accept),
      }
  }
}

impl AsyncRead for HyperStream {
  fn poll_read(
      self: Pin<&mut Self>,
      cx: &mut Context,
      buf: &mut ReadBuf,
  ) -> Poll<io::Result<()>> {
      // info!("impl AsyncRead for HyperStream");
      let pin = self.get_mut();
      match pin.state {
          State::Handshaking(ref mut accept) => match ready!(Pin::new(accept).poll(cx)) {
              Ok(mut stream) => {
                  let result = Pin::new(&mut stream).poll_read(cx, buf);
                  pin.state = State::Streaming(stream);
                  result
              }
              Err(err) => Poll::Ready(Err(err)),
          },
          State::Streaming(ref mut stream) => Pin::new(stream).poll_read(cx, buf),
      }
  }
}

impl AsyncWrite for HyperStream {
  fn poll_write(
      self: Pin<&mut Self>,
      cx: &mut Context<'_>,
      buf: &[u8],
  ) -> Poll<io::Result<usize>> {
      let pin = self.get_mut();
      match pin.state {
          State::Handshaking(ref mut accept) => match ready!(Pin::new(accept).poll(cx)) {
              Ok(mut stream) => {
                  let result = Pin::new(&mut stream).poll_write(cx, buf);
                  pin.state = State::Streaming(stream);
                  result
              }
              Err(err) => Poll::Ready(Err(err)),
          },
          State::Streaming(ref mut stream) => Pin::new(stream).poll_write(cx, buf),
      }
  }

  fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
      match self.state {
          State::Handshaking(_) => Poll::Ready(Ok(())),
          State::Streaming(ref mut stream) => Pin::new(stream).poll_flush(cx),
      }
  }

  fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
      match self.state {
          State::Handshaking(_) => Poll::Ready(Ok(())),
          State::Streaming(ref mut stream) => Pin::new(stream).poll_shutdown(cx),
      }
  }
}

impl Accept for HyperAcceptor {
  type Conn = HyperStream;
  type Error = io::Error;

  fn poll_accept(
      self: Pin<&mut Self>,
      cx: &mut Context<'_>,
  ) -> Poll<Option<Result<Self::Conn, Self::Error>>> {
      let pin = self.get_mut();
      match ready!(Pin::new(&mut pin.incoming).poll_accept(cx)) {
          Some(Ok(sock)) => Poll::Ready(Some(Ok(HyperStream::new(sock, pin.config.clone())))),
          Some(Err(e)) => Poll::Ready(Some(Err(e))),
          None => Poll::Ready(None),
      }
  }
}

async fn handle_https_service(req: Request<Body>) -> Result<Response<Body>, http::Error> {
  if req.uri().path().contains("..") {
    let builder = Response::builder()
      .header("Alt-Svc", "h3=\":443\"; ma=2592000")
      .status(StatusCode::NOT_FOUND);
    return builder.body(Body::empty());
  }

  info!("{:?}", req.method());

  let builder = Response::builder()
  .header("Alt-Svc", "h3=\":443\"; ma=2592000")
  .status(StatusCode::NOT_FOUND);

  if req.method() == Method::GET {
    let (mut resp, body) = handle_request_get(&req, false).await?;
    resp = resp.header("Alt-Svc", "h3=\":443\"; ma=2592000");
    if req.headers().contains_key("Another-Header") {
        resp = resp.header("Another-Header", "Ack");
    }
    resp.body(Body::from(body))
  } else if req.method() == Method::POST {
    let init1 = UrlPatternInit {
      pathname: Some("/upload/:id/*".to_owned()),
      ..Default::default()
    };
    let init2 = UrlPatternInit {
      pathname: Some("/upload/:id".to_owned()),
      ..Default::default()
    };
    let init3 = UrlPatternInit {
      pathname: Some("/profile/:id".to_owned()),
      ..Default::default()
    };
    
    let pattern1 = <UrlPattern>::parse(init1).unwrap();
    let pattern2 = <UrlPattern>::parse(init2).unwrap();
    let pattern3 = <UrlPattern>::parse(init3).unwrap();
    let url = req.uri().to_string().parse().unwrap();
    let input = UrlPatternMatchInput::Url(url);
    // let result = pattern.exec(UrlPatternMatchInput::Url(url)).unwrap().unwrap();
    if let Ok(Some(result)) = pattern1.exec(input.clone()){
      return process_upload(result.pathname.groups.get("id").unwrap().to_string(), builder, req).await;
    }else if let Ok(Some(result)) = pattern2.exec(input.clone()){
      println!("uploadID: {}",result.pathname.groups.get("id").unwrap());
    }else if let Ok(Some(result)) = pattern3.exec(input.clone()){
      println!("profileID: {}",result.pathname.groups.get("id").unwrap());
    }else{
      println!("** error **");
    }
    return builder.body(Body::from(""));
  } else {
    let builder = Response::builder()
      .header("Alt-Svc", "h3=\":443\"; ma=2592000")
      .status(StatusCode::NOT_FOUND);
    builder.body(Body::empty())
  }
}

//uploadIDを表示させる関数
async fn process_upload(id:String, builder:http::response::Builder, req: Request<Body>) -> Result<Response<Body>, http::Error>{
  println!("uploadID: {}",id);
  let content_type_option = req.headers().get("content-type");
  if content_type_option.is_none() {
    return builder.body(Body::from(""));
  }
  let content_type = content_type_option.unwrap();
  let mime_type_result: Result<mime::Mime, _> = match content_type.to_str() {
      Ok(s) => s
          .parse()
          .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err)),
      Err(err) => Err(std::io::Error::new(std::io::ErrorKind::Other, err)),
  };
  if mime_type_result.is_err() {
    return builder.body(Body::from(""));
  }
  let mime_type = mime_type_result.unwrap();
  if mime_type.essence_str() != "multipart/form-data" {
      return builder.body(Body::from(""));
  }
  let a = parse_mpart(req,mime_type).await;
  dbg!(&a);
  return builder.status(StatusCode::OK).body(Body::from(format!("uploadID: {}",id)));
}

// multipartをパースしてhashmapにして返す関数
// ファイルがアップロードされたときはuploadフォルダに保存しhashmapにはそのファイル名を入れる
// アップロードされたファイル名は元のファイル名は使わずに年月日時分秒のファイル名としている
async fn parse_mpart(req: Request<Body>, mime_type: Mime) -> HashMap<String, String>{
  let mut a = HashMap::new();
  let boundary = mime_type.get_param("boundary").map(|v| v.to_string()).ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "boundary not found")).unwrap();
  let (_parts, body) = req.into_parts();
  let mut multipart_stream = mpart_async::server::MultipartStream::new(boundary, body);
  while let Ok(Some(mut field)) = multipart_stream.try_next().await {
    let name = field.name().unwrap().to_string();
    if let Ok(_filename) = field.filename() {
      const UPLOAD_DIRNAME: &str = "upload";
      if fs::create_dir_all(UPLOAD_DIRNAME).is_err(){
        println!("** ディレクトリの作成失敗 **");
        continue;
      }      
      let filename = chrono::Utc::now().format("%Y%m%d%H%M%S").to_string();
      let filename = format!("{UPLOAD_DIRNAME}/{filename}.dat");
      let mut writer = BufWriter::new(File::create(&filename).unwrap());
      let mut bufferlen: i64 = 0;
      while let Ok(Some(bytes)) = field.try_next().await {
        bufferlen += bytes.len() as i64;
        writer.write(&bytes).unwrap();
      }
      a.insert(name, filename);
    }else{
      let mut buffer = BytesMut::new();
      while let Ok(Some(bytes)) = field.try_next().await {
        buffer.put(bytes);
      }
      let value = String::from_utf8(buffer.to_vec()).unwrap();
      a.insert(name, value);
    }
    
  }
  return a;
}