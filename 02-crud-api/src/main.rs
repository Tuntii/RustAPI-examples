// Run with: cargo run -p crud-api
// Then visit: http://127.0.0.1:3000/docs
//
// Lesson: CRUD endpoints, shared state via Arc<RwLock>, typed extractors,
//         and proper extractor ordering (body extractor goes last).

use rustapi_rs::prelude::*;
use rustapi_rs::{delete, get, post, put, summary, tag};
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
// Error type
// ---------------------------------------------------------------------------

#[derive(ApiError)]
enum NoteError {
    #[error(status = 404, code = "NOT_FOUND", message = "Note not found")]
    NotFound,
}

// ---------------------------------------------------------------------------
// Handlers — zero .route() calls needed
// ---------------------------------------------------------------------------

#[get("/notes")]
#[tag("notes")]
#[summary("List all notes")]
async fn list_notes(State(state): State<AppState>) -> Json<Vec<Note>> {
    let notes = state.notes.read().await;
    let mut items: Vec<_> = notes.values().cloned().collect();
    items.sort_by_key(|n| n.id);
    Json(items)
}

#[post("/notes")]
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

#[get("/notes/{id}")]
#[tag("notes")]
#[summary("Get a note by ID")]
async fn get_note(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<Note>, NoteError> {
    state
        .notes
        .read()
        .await
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(NoteError::NotFound)
}

#[put("/notes/{id}")]
#[tag("notes")]
#[summary("Update a note")]
// NOTE: body extractor (Json) must come last in the signature.
async fn update_note(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(payload): Json<UpdateNote>,
) -> Result<Json<Note>, NoteError> {
    let mut notes = state.notes.write().await;
    let note = notes.get_mut(&id).ok_or(NoteError::NotFound)?;
    if let Some(t) = payload.title {
        note.title = t;
    }
    if let Some(b) = payload.body {
        note.body = b;
    }
    Ok(Json(note.clone()))
}

#[delete("/notes/{id}")]
#[tag("notes")]
#[summary("Delete a note")]
async fn delete_note(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<NoContent, NoteError> {
    if state.notes.write().await.remove(&id).is_some() {
        Ok(NoContent)
    } else {
        Err(NoteError::NotFound)
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
    println!(" -> GET    http://127.0.0.1:3000/__rustapi/dashboard");

    // Only state is provided — all routes are auto-discovered from the macros above.
    RustApi::auto()
        .state(state)
        .dashboard(DashboardConfig::new())
        .run("127.0.0.1:3000")
        .await
}
