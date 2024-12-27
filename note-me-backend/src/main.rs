mod note;

mod server;
use crate::server::Server;

#[tokio::main]
async fn main() {
    let server = Server::new();
    let ip = String::from("0.0.0.0");
    server.serve(&ip, 8080).await;
}
