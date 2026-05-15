// Run with: cargo run -p crud-api
// Then visit: http://127.0.0.1:3000/docs
//
// Lesson: CRUD endpoints, shared state via Arc<RwLock>, typed extractors,
//         and proper extractor ordering (body extractor goes last).

use rustapi_rs::prelude::*;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};
use tokio::sync::RwLock;

// ---------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct AppState {
    next_id: Arc<AtomicU64>,
    notes: Arc<RwLock<HashMap<u64, Note>>>,
}

// ---------------------------------------------------------------------------
// Models
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Schema)]
struct Note {
    id: u64,
    title: String,
    body: String,
}

/// Request body for creating a note.
#[derive(Debug, Deserialize, Schema)]
struct CreateNote {
    title: String,
    body: String,
}

/// Request body for updating a note (all fields optional).
#[derive(Debug, Deserialize, Schema)]
struct UpdateNote {
    title: Option<String>,
    body: Option<String>,
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

#[tag("notes")]
#[summary("List all notes")]
async fn list_notes(State(state): State<AppState>) -> Json<Vec<Note>> {
    let notes = state.notes.read().await;
    let mut items: Vec<_> = notes.values().cloned().collect();
    items.sort_by_key(|n| n.id);
    Json(items)
}

#[tag("notes")]
#[summary("Create a note")]
// NOTE: body extractor (Json) must come last in the signature.
async fn create_note(
    State(state): State<AppState>,
    Json(payload): Json<CreateNote>,
) -> Created<Note> {
    let id = state.next_id.fetch_add(1, Ordering::SeqCst);
    let note = Note {
        id,
        title: payload.title,
        body: payload.body,
    };
    state.notes.write().await.insert(id, note.clone());
    Created(note)
}

#[tag("notes")]
#[summary("Get a note by ID")]
async fn get_note(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<Note>, NotFound> {
    let notes = state.notes.read().await;
    notes
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(NotFound)
}

#[tag("notes")]
#[summary("Update a note")]
// NOTE: body extractor (Json) must come last in the signature.
async fn update_note(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(payload): Json<UpdateNote>,
) -> Result<Json<Note>, NotFound> {
    let mut notes = state.notes.write().await;
    let note = notes.get_mut(&id).ok_or(NotFound)?;
    if let Some(t) = payload.title {
        note.title = t;
    }
    if let Some(b) = payload.body {
        note.body = b;
    }
    Ok(Json(note.clone()))
}

#[tag("notes")]
#[summary("Delete a note")]
async fn delete_note(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<NoContent, NotFound> {
    let mut notes = state.notes.write().await;
    if notes.remove(&id).is_some() {
        Ok(NoContent)
    } else {
        Err(NotFound)
    }
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let state = AppState {
        next_id: Arc::new(AtomicU64::new(1)),
        notes: Arc::new(RwLock::new(HashMap::new())),
    };

    println!("Starting crud-api example…");
    println!(" -> GET    http://127.0.0.1:3000/notes");
    println!(" -> POST   http://127.0.0.1:3000/notes");
    println!(" -> GET    http://127.0.0.1:3000/notes/{{id}}");
    println!(" -> PUT    http://127.0.0.1:3000/notes/{{id}}");
    println!(" -> DELETE http://127.0.0.1:3000/notes/{{id}}");
    println!(" -> GET    http://127.0.0.1:3000/docs");

    RustApi::new()
        .swagger_ui("/docs")
        .state(state)
        .route("/notes", get(list_notes).post(create_note))
        .route("/notes/{id}", get(get_note).put(update_note).delete(delete_note))
        .run("127.0.0.1:3000")
        .await
}
