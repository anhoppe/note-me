use std::str::FromStr;

use axum::{routing::put, response::IntoResponse, Router, Json};

mod note;
use crate::note::Note;

mod server;
use crate::server::Server;

#[tokio::main]
async fn main() {
    // In-memory storage for simplicity. For production, consider a database.
    // let notes = Arc::new(Mutex::new(Vec::new()));

    // let app = Router::new()
    //     .route("/notes", put(update_note));

    // let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
    //     .await
    //     .unwrap();
    // // tracing::debug!("listening on {}", listener.local_addr().unwrap());

    // axum::serve(listener, app).await.unwrap();

    let server = Server::new();
    let ip = String::from("127.0.0.1");
    server.serve(&ip, 3000).await;
}

// async fn update_note(Json(note): Json<Note>) -> impl IntoResponse {
//     println!("Received Note wtf: title={}, text={}, date={}", note.title, note.text, note.date);
    
//     // Here, you could add logic to save the note to a database or handle it as needed.

//     http::StatusCode::OK
// }