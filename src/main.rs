use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let port = 8000;
    let address = format!("http://127.0.0.1:{}", port);
    let listener =
        TcpListener::bind(address).unwrap_or_else(|_| panic!("Failed to bind to port: {}", port));
    run(listener)?.await
}
