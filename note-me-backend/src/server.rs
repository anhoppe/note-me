use axum::{extract::Path, routing::put, routing::get, response::IntoResponse, Router, Json};
use http::{Method, StatusCode};
use http::header::{CONTENT_TYPE};
use std::sync::{Arc, Mutex};
use tower_http::cors::{CorsLayer, Any};

use crate::note::Note;

pub struct Server {
    notes: Vec<Note>,
}

impl Server {
    pub fn new() -> Self {
        Server { notes: Vec::new() }
    }

    pub async fn serve(self, ip: &str, port: i32) {
        let cors = CorsLayer::new()
        // allow `GET`, `POST`, and `PUT` when accessing the resource
        .allow_methods([Method::GET, Method::POST, Method::PUT])
        // allow requests from any origin
        .allow_origin(Any) // Only allow requests from your React app
        //.allow_origin(Origin::any()) // NOT RECOMMENDED FOR PRODUCTION
        .allow_headers([CONTENT_TYPE]);

        let server = Arc::new(Mutex::new(self));

        let app = Router::new()
        .route(
            "/notes",
            put({
                println!("put");
                let server = Arc::clone(&server);
                move |note| Self::update_note(server, note)
            }),
        )
        .route(
            "/notes",
            get({
                println!("get");
                let server = Arc::clone(&server);
                move || Self::get_notes(server)
            }),
        )
        .route(
            "/notes/:id",
            get({
                println!("get/:id");
                let server = Arc::clone(&server);
                move |id| Self::get_note_by_id(server, id)
            }),
        )
        .layer(cors);

        let address = format!("{}:{}", ip, port);
        let listener = tokio::net::TcpListener::bind(&address)
            .await
            .expect("Failed to bind");

        println!("Server running on {}", address);
        axum::serve(listener, app).await.unwrap();
    }

    async fn get_note_by_id(server: Arc<Mutex<Self>>, Path(id): Path<u64>) -> impl IntoResponse {
        let server = server.lock().unwrap();
        let notes = server.notes.clone();
        if let Some(note) = notes.iter().find(|note| note.id == id).cloned() {
            Json(note).into_response()
        } else {
            (StatusCode::NOT_FOUND, "Note not found").into_response()
        }
    }
    
    async fn get_notes(server: Arc<Mutex<Self>>) -> impl IntoResponse {
        let server = server.lock().unwrap();
        let notes = server.notes.clone();
        Json(notes)
    }
    
    async fn update_note(server: Arc<Mutex<Self>>, Json(note): Json<Note>) -> impl IntoResponse {
        println!("update_note");

        let mut server = server.lock().unwrap();
        println!(
            "Received note: id={}, title={}, text={}, date={}",
            note.id, note.title, note.text, note.created_at
        );

        // Update the notes in the server
        server.notes.push(note);

        http::StatusCode::OK
    }
}
