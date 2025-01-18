use axum::{extract::Path, extract::Query, routing::post, routing::put, routing::get, response::IntoResponse, Router, Json};
use http::{Method, StatusCode};
use http::header::{CONTENT_TYPE};
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use tower_http::cors::{CorsLayer, Any};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};
use tracing::info;

use crate::note::Note;

pub struct Server {
    notes: Vec<Note>,
}

#[derive(Deserialize)]
struct UserId {
    user_id: String,
}

impl Server {
    pub fn new() -> Self {
        Server { notes: Vec::new() }
    }

    pub async fn serve(self, ip: &str, port: i32) {
        tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_tracing=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

        info!("Starting server..."); // Now you can use tracing macros

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

            }),
        )
        .route(
            "/notes",
            get({
                let server = Arc::clone(&server);
                move |user_id: Query<UserId>| Self::get_notes(server, user_id)
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

    async fn get_notes(server: Arc<Mutex<Self>>, Query(user_id): Query<UserId>) -> impl IntoResponse {
        let server = server.lock().unwrap();

        println!("All notes:");
        for note in server.notes.iter()  {
            println!("{}: {}", note.user_id, note.title);
        }

        let notes: Vec<Note> = server.notes.clone()
        .into_iter()
        .filter(|note| note.user_id == user_id.user_id)
        .collect();

        println!("Filtered by user:");
        for note in notes.iter()  {
            println!("{}", note.title);
        }
        
        println!(
            "Num notes for user {}: {}",
            user_id.user_id,
            notes.len()
        );
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

        if let Some(n) = server.notes.iter_mut().find(|n| n.id == id && n.user_id == note.user_id) {
            n.title = note.title;
            n.text = note.text;
        } else {
            return http::StatusCode::NOT_FOUND;
        }

        http::StatusCode::OK
    }
}
