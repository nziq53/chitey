use std::{net::SocketAddr, sync::Arc};

use http::{Request, Response, StatusCode};
use hyper::{Body, Server, service::{make_service_fn, service_fn}, server::conn::AddrStream};

#[derive(Clone)]
pub struct HttpServerOpt {
  pub listen: SocketAddr,
  pub redirect: Arc<String>,
}

pub async fn launch_http_server<F> (http_server_opt: HttpServerOpt, func: F) -> Result<(), Box<dyn std::error::Error>> 
where
    F: Fn()
{
  let HttpServerOpt{listen, redirect} = http_server_opt;

  // 80ポートにhttpアクセスが来た時にリダイレクトしたりするため
  println!("redirect to {}", redirect.clone());
  let http_make_service = make_service_fn(move |_conn: &AddrStream| {
      let location = redirect.clone();
      let service = service_fn(move |req| {
          redirect_to_https(location.clone(), req)
      });
      async move { Ok::<_, http::Error>(service) }
  });
  let http_server = Server::bind(&listen).serve(http_make_service);
  println!("Listening on http://{}", listen);
  func();
  let _ = http_server.await?;

  Ok(())
}

async fn redirect_to_https(
  location: impl std::ops::Deref<Target=String>,
  _req: Request<Body>,
) -> Result<Response<Body>, http::Error> {
  let location = location.deref();
  let mut builder = Response::builder();
  if location.as_str() != "non" {
      builder = builder
          .status(StatusCode::PERMANENT_REDIRECT)
          .header("Location", location);
  }
  // info!("location {}", location);
  builder.body(Body::empty())
}
