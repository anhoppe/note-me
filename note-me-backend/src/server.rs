use axum::{extract::Path, routing::post, routing::put, routing::get, response::IntoResponse, Router, Json};
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
            post({
                println!("post");
                let server = Arc::clone(&server);
                move |note| Self::create_note(server, note)
            }),
        )
        .route(
            "/notes/:id",
            put({
                println!("put");
                let server = Arc::clone(&server);
                move |id: Path<u64>, note: Json<Note>| Self::update_note_by_id(server, id, note)
                // move |note| print!("bar")
                // move |Path(id): Path<u64>, Json(note): Json<Note>| print!("bar")
                    //Self::update_note_by_id(server, id, note);

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

    async fn create_note(server: Arc<Mutex<Self>>, Json(note): Json<Note>) -> impl IntoResponse {
        println!("create_note");

        let mut server = server.lock().unwrap();
        println!(
            "Received note: id={}, title={}, text={}, date={}",
            note.id, note.title, note.text, note.created_at
        );

        // Update the notes in the server
        server.notes.push(note);

        http::StatusCode::OK
    }

    async fn update_note_by_id(server: Arc<Mutex<Self>>, Path(id): Path<u64>, Json(note): Json<Note>) -> impl IntoResponse {
        println!("update_note_by_id");

        let mut server = server.lock().unwrap();

        if let Some(n) = server.notes.iter_mut().find(|n| n.id == id) {
            n.title = note.title;
            n.text = note.text;
        } else {
            return http::StatusCode::NOT_FOUND;
        }

        http::StatusCode::OK
    }
}
