use std::net::SocketAddr;

use http::{Request, Response, StatusCode};
use hyper::{Body, Server, service::{make_service_fn, service_fn}, server::conn::AddrStream};

#[derive(Clone)]
pub struct HttpServerOpt {
  pub listen: SocketAddr,
  pub redirect: Option<String>,
}

pub async fn launch_http_server<F> (http_server_opt: HttpServerOpt, func: F) -> Result<(), Box<dyn std::error::Error>> 
where
    F: Fn()
{
  let HttpServerOpt{listen, redirect} = http_server_opt;

  // 80ポートにhttpアクセスが来た時にリダイレクトしたりするため
  if let Some(redirect) = redirect {
    println!("redirect to {}", redirect);
    let http_make_service = make_service_fn(move |_conn: &AddrStream| {
        let location = redirect.clone().to_owned();
        let service = service_fn(move |req| {
            redirect_to_https(location.clone(), req)
        });
        async move { Ok::<_, http::Error>(service) }
    });
    let http_server = Server::bind(&listen).serve(http_make_service);
    println!("Listening on http://{}", listen);
    func();
    let _ = http_server.await?;
  
  } else {
    let http_make_service = make_service_fn(move |_conn: &AddrStream| {
        let service = service_fn(move |req| {
            not_redirect_to_https(req)
        });
        async move { Ok::<_, http::Error>(service) }
    });
    let http_server = Server::bind(&listen).serve(http_make_service);
    println!("Listening on http://{}", listen);
    func();
    let _ = http_server.await?;
  
  }
  Ok(())
}

async fn redirect_to_https(
  location: String,
  _req: Request<Body>,
) -> Result<Response<Body>, http::Error> {
  let mut builder = Response::builder();
    builder = builder
        .status(StatusCode::PERMANENT_REDIRECT)
        .header("Location", location);
  // info!("location {}", location);
  builder.body(Body::empty())
}

async fn not_redirect_to_https(
  _req: Request<Body>,
) -> Result<Response<Body>, http::Error> {
  Response::builder().body(Body::empty())
}
