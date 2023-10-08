use std::net::SocketAddr;

use bytes::Bytes;
use http::{Request, Response, StatusCode, HeaderValue};
use hyper::{Body, Server, service::{make_service_fn, service_fn}, server::conn::AddrStream};
use urlpattern::UrlPatternMatchInput;

use crate::web_server::{ChiteyError, Factories};

use super::util::throw_chitey_internal_server_error;


#[derive(Clone)]
pub struct HttpServerOpt {
  pub listen: SocketAddr,
  pub redirect: Option<String>,
}

pub async fn launch_http_server<F> (http_server_opt: HttpServerOpt, func: F, factories: Factories) -> Result<(), Box<dyn std::error::Error>>
where
    F: Fn()
{
    let HttpServerOpt{listen, redirect} = http_server_opt;

    // 80ポートにhttpアクセスが来た時にリダイレクトしたりするため
    if let Some(redirect) = redirect {
        // println!("redirect to {}", redirect);
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
            let factories = factories.clone();
            let service = service_fn(move |req| {
                not_redirect_to_https_wrap(req, factories.clone(), listen.to_string())
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

#[inline]
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

#[inline]
async fn not_redirect_to_https_wrap(
    req: Request<Body>,
    factories: Factories,
    listen: String,
) -> Result<Response<Body>, ChiteyError> {
    match not_redirect_to_https(req, factories.clone(), listen.to_string()).await {
        Ok(v) => Ok(v),
        Err(e) => {
            tracing::error!("http: {}", e);
            Err(e)
        },
    }
}

#[inline]
async fn not_redirect_to_https(
  req: Request<Body>,
  factories: Factories,
  listen: String,
) -> Result<Response<Body>, ChiteyError> {
    if req.uri().path().contains("..") {
        let builder = Response::builder()
        .header("Alt-Svc", "h3=\":443\"; ma=2592000")
        .status(StatusCode::NOT_FOUND);
        return match builder.body(Body::empty()) {
            Ok(v) => Ok(v),
            Err(e) => Err(ChiteyError::InternalServerError(e.to_string())),
        }
    }

    let input = UrlPatternMatchInput::Url(throw_chitey_internal_server_error((format!("http://{}{}",listen , &req.uri().to_string())).parse())?);
    {
        let method = req.method().clone();
        let req_contain_key = req.headers().contains_key("Another-Header");
        for (res, factory) in factories.factories {
            // GET && POST
            if res.guard == method {
                if let Ok(Some(_)) = res.rdef.exec(input.clone()) {
                    let factory_loc = factory.lock().await;
                    if factory_loc.analyze_types(input.clone()) {
                        return match factory_loc.handler_func(input.clone(), (req, false, factories.contexts.clone())).await {
                            Ok(mut resp) => {
                                if req_contain_key {
                                    resp.headers_mut().append("Another-Header", HeaderValue::from_static("Ack"));
                                }
                                resp.headers_mut().append("Alt-Svc", HeaderValue::from_static("h3=\":443\"; ma=2592000"));
                                Ok(resp)
                            },
                            Err(e) => Err(ChiteyError::InternalServerError(e.to_string())),
                        }
                    }
                };
            }
        }
    }

    let builder = Response::builder()
        .header("Alt-Svc", "h3=\":443\"; ma=2592000")
        .status(StatusCode::NOT_FOUND);

    match builder.body(Body::from(Bytes::copy_from_slice(b"page not found"))) {
        // match builder.body(Body::empty()) {
        Ok(v) => Ok(v),
        Err(e) => Err(ChiteyError::InternalServerError(e.to_string())),
    }
}
