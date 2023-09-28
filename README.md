このフレームワークはactix-webを軽く参考にしています。
# 何これ
hyperとh3を統合した雑なWebサーバフレームワークです。<br>

httpはリダイレクト、https、http3を同じように書くことができます。<br>

簡単に書けるのも特徴。<br>

プルリクはいつでも受け付けています。<br>

# 使い方

```rustuse chitey::{get, Responder, WebServer, post, Certs, Request, ChiteyError, http::Response, Bytes};
#[get("/:id/:name")]
async fn greet((id, name): (u32, String), _req: Request) -> Responder {
    Ok((Response::builder(), Bytes::from(format!("Hello {}! id:{}", name, id))))
}

#[get("/:id/:name")]
async fn greet_not_number((id, name): (String, String), _req: Request) -> Responder {
    Ok((Response::builder(), Bytes::from(format!("HelloNotNumberId {}! id:{}", name, id))))
}

#[get("/")]
async fn home((): (), _req: Request) -> Responder {
    Ok((Response::builder(), Bytes::copy_from_slice(b"source")))
}

#[get("/all/:all")]
async fn all_all_path((): (), _req: Request) -> Responder {
    Ok((Response::builder(), Bytes::copy_from_slice(b"source")))
}

#[get("/:all")]
async fn all_path((all,): (String,), _req: Request) -> Responder {
    Ok((Response::builder(), Bytes::from(format!("All String: {}", all))))
}

#[get("/**")]
async fn notfound((): (), _req: Request) -> Responder {
    Ok((Response::builder(), Bytes::copy_from_slice(b"404 not found")))
}

#[post("/**")]
async fn notfoundpost((): (), _req: Request) -> Responder {
    Ok((Response::builder(), Bytes::copy_from_slice(b"404 not found")))
}

#[tokio::main]
async fn main() -> Result<(), ChiteyError> {
    println!("Hello, world!");
    WebServer::new()
    .bind("localhost:18080").unwrap()
    .tls_bind("localhost:18443").unwrap()
    .tls(Certs { cert: "server.cert".into(), key: "server.key".into() })
    .service(greet)
    .service(greet_not_number)
    .service(home)
    .service(all_all_path)
    .service(all_path)
    .service(notfound)
    .service(notfoundpost)
    .run()
    .await
}
```

## バックグラウンドで立てているサーバをkillするには
```rust
chitey::kill_server() -> Result<(), ChiteyError>;
```
を実行することによってキルできます。（各OSのtmpに保存しているので、最新のみです）

certファイルをアップデートしたときに勝手に更新しません！！
再起動しかないです。プルリクできる方はどうぞ
