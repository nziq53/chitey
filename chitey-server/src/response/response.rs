
use bytes::Bytes;
use http::{Request, Response, StatusCode};

pub async fn handle_request_get<T>(req: &Request<T>, isHttp3: bool) -> Result<(http::response::Builder, bytes::Bytes), http::Error> {
  let builder = Response::builder();
  let ret = if isHttp3 {
    Bytes::copy_from_slice(b"source and http3")
  } else {
    Bytes::copy_from_slice(b"source")
  };
  Ok((builder, ret))
}

pub async fn handle_request_post<T>(req: &Request<T>, isHttp3: bool) -> Result<(http::response::Builder, bytes::Bytes), http::Error> {
  let builder = Response::builder()
    .header("Alt-Svc", "h3=\":443\"; ma=2592000")
    .status(StatusCode::OK);
  Ok((builder, Bytes::copy_from_slice(b"http2")))
}
