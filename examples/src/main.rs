use chitey::{get};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}
