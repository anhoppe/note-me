mod note;

mod server;
use crate::server::Server;

#[tokio::main]
async fn main() {
    let server = Server::new();
    let ip = String::from("127.0.0.1");
    server.serve(&ip, 3000).await;
}
