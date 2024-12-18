use axum::{routing::put, routing::get, response::IntoResponse, Router, Json};
use std::sync::{Arc, Mutex};

use crate::note::Note;

pub struct Server {
    notes: Vec<Note>,
}

impl Server {
    pub fn new() -> Self {
        Server { notes: Vec::new() }
    }

    pub async fn serve(self, ip: &str, port: i32) {
        let server = Arc::new(Mutex::new(self));

        let app = Router::new()
        .route(
            "/notes",
            put({
                let server = Arc::clone(&server);
                move |note| Self::update_note(server, note)
            }),
        )
        .route(
            "/notes",
            get({
                let server = Arc::clone(&server);
                move || Self::get_notes(server)
            },)
        );

        let address = format!("{}:{}", ip, port);
        let listener = tokio::net::TcpListener::bind(&address)
            .await
            .expect("Failed to bind");

        println!("Server running on {}", address);
        axum::serve(listener, app).await.unwrap();
    }

    
    async fn get_notes(server: Arc<Mutex<Self>>) -> impl IntoResponse {
        let server = server.lock().unwrap();
        let count = server.notes.len();
        Json(count)
    }
    
    async fn update_note(server: Arc<Mutex<Self>>, Json(note): Json<Note>) -> impl IntoResponse {
        let mut server = server.lock().unwrap();
        println!(
            "Received note: title={}, text={}, date={}",
            note.title, note.text, note.date
        );

        // Update the notes in the server
        server.notes.push(note);

        http::StatusCode::OK
    }
}
