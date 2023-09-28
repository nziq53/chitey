use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::net::SocketAddr;
use std::sync::Arc;
use bytes::Buf;
use bytes::BufMut;
use bytes::Bytes;
use bytes::BytesMut;
use futures_util::TryStreamExt;
use h3::error::ErrorLevel;
use h3::quic::BidiStream;
use h3::server::RequestStream;
use http::Method;
use http::Request;
use http::StatusCode;
use hyper::Body;
use tracing::{error, info, trace_span};
use urlpattern::UrlPatternMatchInput;

use crate::guard::Guard;
use crate::response::response::handle_request_get;
use crate::server::http3_stream_wrapper::StreamWrapper;
use crate::web_server::ChiteyError;
use crate::web_server::Factories;


use super::util::TlsCertsKey;

#[derive(Clone)]
pub struct Http3ServerOpt {
    pub listen: SocketAddr,
}

pub async fn launch_http3_server(
    tls_cert_key: TlsCertsKey,
    http3_server_opt: Http3ServerOpt,
    factories: Factories,
) -> Result<(), Box<dyn std::error::Error>> {
    let TlsCertsKey { certs, key } = tls_cert_key;
    let Http3ServerOpt { listen } = http3_server_opt;

    let tls_config = {
        let mut tls_config = rustls::ServerConfig::builder()
            .with_safe_default_cipher_suites()
            .with_safe_default_kx_groups()
            .with_protocol_versions(&[&rustls::version::TLS13])
            .unwrap()
            .with_no_client_auth()
            .with_single_cert(certs.clone(), key.clone())?;
        tls_config.max_early_data_size = u32::MAX;
        let alpn: &[u8] = b"h3";
        tls_config.alpn_protocols = vec![alpn.into()];
        tls_config
    };

    let server_config = quinn::ServerConfig::with_crypto(Arc::new(tls_config));
    let endpoint = quinn::Endpoint::server(server_config, listen)?;

    while let Some(new_conn) = endpoint.accept().await {
        #[cfg(debug_assertions)]
        trace_span!("New connection being attempted");

        let factories = factories.clone();
        tokio::spawn(async move {
            match new_conn.await {
                Ok(conn) => {
                    #[cfg(debug_assertions)]
                    info!("new connection established");

                    let mut h3_conn = h3::server::Connection::new(h3_quinn::Connection::new(conn))
                        .await
                        .unwrap();

                    loop {
                        match h3_conn.accept().await {
                            Ok(Some((req, stream))) => {
                                #[cfg(debug_assertions)]
                                info!("new request: {:#?}", req);

                                let factories = factories.clone();
                                tokio::spawn(async move {
                                    // if let Err(e) = handle_request_http3(req, stream).await {
                                    //     #[cfg(debug_assertions)]
                                    //     error!("handling request failed: {}", e);
                                    // };
                                    let _ = handle_request_http3(req, stream, factories).await;
                                });
                            }

                            // indicating no more streams to be received
                            Ok(None) => {
                                break;
                            }

                            Err(err) => {
                                #[cfg(debug_assertions)]
                                error!("error on accept {}", err);
                                match err.get_error_level() {
                                    ErrorLevel::ConnectionError => break,
                                    ErrorLevel::StreamError => continue,
                                }
                            }
                        }
                    }
                }
                Err(err) => {
                    #[cfg(debug_assertions)]
                    error!("accepting connection failed: {:?}", err);
                }
            }
        });
    }

    // shut down gracefully
    // wait for connections to be closed before exiting
    println!("Starting to serve on https://{}.", listen);
    endpoint.wait_idle().await;

    Ok(())
}

pub async fn handle_request_http3<T>(
    req: Request<()>,
    mut stream: RequestStream<T, Bytes>,
    factories: Factories,
) -> Result<(), ChiteyError>
where
    T: BidiStream<Bytes> + 'static + Send + Sync,
{
    if req.uri().path().contains("..") {
        let resp = http::Response::builder().status(StatusCode::NOT_FOUND).body(()).unwrap();

        match stream.send_response(resp).await {
            Ok(_) => {
                #[cfg(debug_assertions)]
                info!("successfully respond to connection");
            }
            Err(err) => {
                #[cfg(debug_assertions)]
                error!("unable to send response to connection peer: {:?}", err);
            }
        }
        stream.send_data(Bytes::copy_from_slice(b"page not found")).await;
        return match stream.finish().await {
            Ok(_) => Ok(()),
            Err(e) =>Err(ChiteyError::InternalServerError(e.to_string())),
        }
    }
    let url = req.uri().to_string().parse().unwrap();
    let input = UrlPatternMatchInput::Url(url);

    let method = req.method().clone();
    let req_contain_key = req.headers().contains_key("Another-Header");

    let (mut send_stream, recv_stream) = stream.split();
    let stm: StreamWrapper<T> = StreamWrapper::new(recv_stream);
    let req = req.map(|_| Body::wrap_stream(stm));

    for (res, factory) in factories.factories {
        // GET
        if res.guard == Guard::Get && method == Method::GET {
          if let Ok(Some(_)) = res.rdef.exec(input.clone()) {
            return match factory.lock().await.handler_func(input.clone(), (req, false)).await {
              Ok((mut resp, body)) => {
                if req_contain_key {
                  resp = resp.header("Another-Header", "Ack");
                }
                send_stream.send_response(resp.body(()).unwrap()).await;
                send_stream.send_data(body).await;
                match send_stream.finish().await {
                    Ok(_) => Ok(()),
                    Err(e) =>Err(ChiteyError::InternalServerError(e.to_string())),
                }
              },
              Err(e) => Err(ChiteyError::InternalServerError(e.to_string())),
            }
          };
        }
  
        // POST
        if res.guard == Guard::Post && method == Method::POST {
          if let Ok(Some(_)) = res.rdef.exec(input.clone()) {
            return match factory.lock().await.handler_func(input.clone(), (req, false)).await {
              Ok((mut resp, body)) => {
                if req_contain_key {
                  resp = resp.header("Another-Header", "Ack");
                }
                send_stream.send_response(resp.body(()).unwrap()).await;
                send_stream.send_data(body).await;
                match send_stream.finish().await {
                    Ok(_) => Ok(()),
                    Err(e) =>Err(ChiteyError::InternalServerError(e.to_string())),
                }
              },
              Err(e) => Err(ChiteyError::InternalServerError(e.to_string())),
            }
          };
        }
      }


        // let content_type = content_type_option.unwrap();
        // let mime_type_result: Result<mime::Mime, _> = match content_type.to_str() {
        //     Ok(s) => s
        //         .parse()
        //         .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err)),
        //     Err(err) => Err(std::io::Error::new(std::io::ErrorKind::Other, err)),
        // };
        // if mime_type_result.is_err() {
        //     return Ok(());
        // }
        // let mime_type = mime_type_result.unwrap();
        // if mime_type.essence_str() != "multipart/form-data" {
        //     return Ok(());
        // }
        // let boundary = mime_type
        //     .get_param("boundary")
        //     .map(|v| v.to_string())
        //     .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "boundary not found"))?;
        // // if let Ok(Some(post_data)) = stream.recv_data().await{
        // //   info!("{:?}", post_data.chunk());
        // // }
        // let stm: StreamWrapper<T> = StreamWrapper::new(recv_stream);
        // let mut req = req.map(|_| Body::wrap_stream(stm));
        //     let (re, b)  = req.into_parts();

        // let mut multipart_stream = mpart_async::server::MultipartStream::new(
        //     boundary,
        //     b.map_ok(|buf| {
        //         let mut ret = BytesMut::with_capacity(buf.remaining());
        //         ret.put(buf);
        //         ret.freeze()
        //     }),
        // );

        // while let Ok(Some(mut field)) = multipart_stream.try_next().await {
        //     println!("Field name:{}", field.name().unwrap());
        //     if let Ok(filename) = field.filename() {
        //         println!("Field filename:{}", filename);
        //         let mut writer = BufWriter::new(File::create(filename.as_ref()).unwrap());
        //         let mut bufferlen: i64 = 0;
        //         while let Ok(Some(bytes)) = field.try_next().await {
        //             bufferlen += bytes.len() as i64;
        //             writer.write(&bytes).unwrap();
        //         }
        //         println!("Bytes received:{}", bufferlen);
        //     } else {
        //         let mut buffer = BytesMut::new();
        //         while let Ok(Some(bytes)) = field.try_next().await {
        //             buffer.put(bytes);
        //         }
        //         let value: String = String::from_utf8(buffer.to_vec()).unwrap();
        //         println!("{} = {}", field.name().unwrap(), value);
        //     }
        // }

        // let resp = http::Response::builder().status(status).body(()).unwrap();

        // // let post_stream = PostStream

        // match send_stream.send_response(resp).await {
        //     Ok(_) => {
        //         #[cfg(debug_assertions)]
        //         info!("successfully respond to connection");
        //     }
        //     Err(err) => {
        //         #[cfg(debug_assertions)]
        //         error!("unable to send response to connection peer: {:?}", err);
        //     }
        // }

        // if let Ok(Some(post_data)) = recv_stream.recv_data().await{
        //   info!("{:?}", post_data.chunk());
        // }
        // info!("{:?}", req.headers().get("content-type"));
        // match recv_stream.recv_trailers().await {
        //   Ok(v) => match v {
        //     Some(d) => { info!("recv: {:?}", d) },
        //     None => { info!("recv: None"); },
        //   },
        //   Err(v) => { info!("recv: {:?}", v); },
        // };
        // let mut num = 0;
        // loop {
        //   match recv_stream.recv_data().await {
        //     Ok(v) => match v {
        //       Some(d) => {
        //         // info!("{:?}", d.chunk());
        //         // info!("{}", std::str::from_utf8(d.chunk()).unwrap());
        //         num += d.remaining();
        //         // info!("{:?}", d.remaining());
        //       },
        //       None => { info!("None"); break; },
        //     },
        //     Err(v) => { info!("{:?}", v); break; },
        //   }
        //   // tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        // }
        // info!("num: {}", num);
        // match recv_stream.recv_trailers().await {
        //   Ok(v) => match v {
        //     Some(d) => { info!("recv: {:?}", d) },
        //     None => { info!("recv: None"); },
        //   },
        //   Err(v) => { info!("recv: {:?}", v); },
        // };
        // let mut buf = BytesMut::new();
        // buf.extend_from_slice(b"hello world http3");
        // send_stream.send_data(buf.freeze()).await?;

        // Ok(send_stream.finish().await?)
    //     Ok(())
    // } else {
    let resp = http::Response::builder().status(StatusCode::NOT_FOUND).body(()).unwrap();
    send_stream.send_response(resp).await;
    send_stream.send_data(Bytes::copy_from_slice(b"page not found")).await;
    return match send_stream.finish().await {
        Ok(_) => Ok(()),
        Err(e) =>Err(ChiteyError::InternalServerError(e.to_string())),
    }
}
