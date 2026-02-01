//! GraphQL API Example for RustAPI
//!
//! This example demonstrates:
//! - GraphQL queries and mutations
//! - async-graphql integration
//! - Type-safe resolvers
//! - GraphQL playground
//!
//! Run with: cargo run -p graphql-api
//! Then visit: http://127.0.0.1:8080/graphql (GraphQL playground)

use async_graphql::{Context, EmptySubscription, Object, Schema, SimpleObject};
use rustapi_rs::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

// ============================================
// Data Models
// ============================================

#[derive(Debug, Clone, SimpleObject)]
struct Book {
    id: u64,
    title: String,
    author: String,
    year: u32,
}

#[derive(Debug, Clone, SimpleObject)]
struct Author {
    id: u64,
    name: String,
    bio: String,
}

// ============================================
// Database Mock
// ============================================

#[allow(dead_code)]
#[derive(Clone)]
struct Database {
    books: Arc<RwLock<HashMap<u64, Book>>>,
    authors: Arc<RwLock<HashMap<u64, Author>>>,
    next_book_id: Arc<RwLock<u64>>,
    next_author_id: Arc<RwLock<u64>>,
}

impl Database {
    fn new() -> Self {
        let mut books = HashMap::new();
        let mut authors = HashMap::new();

        // Seed data
        books.insert(
            1,
            Book {
                id: 1,
                title: "The Rust Programming Language".to_string(),
                author: "Steve Klabnik".to_string(),
                year: 2018,
            },
        );
        books.insert(
            2,
            Book {
                id: 2,
                title: "Programming Rust".to_string(),
                author: "Jim Blandy".to_string(),
                year: 2021,
            },
        );

        authors.insert(
            1,
            Author {
                id: 1,
                name: "Steve Klabnik".to_string(),
                bio: "Rust core team member".to_string(),
            },
        );

        Self {
            books: Arc::new(RwLock::new(books)),
            authors: Arc::new(RwLock::new(authors)),
            next_book_id: Arc::new(RwLock::new(3)),
            next_author_id: Arc::new(RwLock::new(2)),
        }
    }

    fn get_book(&self, id: u64) -> Option<Book> {
        self.books.read().unwrap().get(&id).cloned()
    }

    fn get_all_books(&self) -> Vec<Book> {
        self.books.read().unwrap().values().cloned().collect()
    }

    fn add_book(&self, title: String, author: String, year: u32) -> Book {
        let mut id_lock = self.next_book_id.write().unwrap();
        let id = *id_lock;
        *id_lock += 1;

        let book = Book {
            id,
            title,
            author,
            year,
        };

        self.books.write().unwrap().insert(id, book.clone());
        book
    }
}

// ============================================
// GraphQL Schema
// ============================================

struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Get a book by ID
    async fn book(&self, ctx: &Context<'_>, id: u64) -> Option<Book> {
        let db = ctx.data::<Database>().unwrap();
        db.get_book(id)
    }

    /// Get all books
    async fn books(&self, ctx: &Context<'_>) -> Vec<Book> {
        let db = ctx.data::<Database>().unwrap();
        db.get_all_books()
    }

    /// Search books by title
    async fn search_books(&self, ctx: &Context<'_>, query: String) -> Vec<Book> {
        let db = ctx.data::<Database>().unwrap();
        db.get_all_books()
            .into_iter()
            .filter(|book| book.title.to_lowercase().contains(&query.to_lowercase()))
            .collect()
    }
}

struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Add a new book
    async fn add_book(
        &self,
        ctx: &Context<'_>,
        title: String,
        author: String,
        year: u32,
    ) -> Book {
        let db = ctx.data::<Database>().unwrap();
        db.add_book(title, author, year)
    }
}

type ApiSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

// ============================================
// Handlers
// ============================================

#[allow(dead_code)]
#[derive(Deserialize, Schema)]
struct GraphQLRequest {
    query: String,
    #[serde(default)]
    variables: Option<String>,
    #[serde(default)]
    operation_name: Option<String>,
}

/// GraphQL response wrapper
#[derive(Serialize, Schema)]
struct GraphQLResponse {
    /// JSON response as string
    response: String,
}

/// GraphQL endpoint
#[rustapi_rs::post("/graphql")]
async fn graphql_handler(
    State(schema): State<ApiSchema>,
    Json(request): Json<GraphQLRequest>,
) -> Json<GraphQLResponse> {
    let query = request.query;
    let response = schema.execute(&query).await;
    let response_str = serde_json::to_string(&response).unwrap();
    
    Json(GraphQLResponse {
        response: response_str,
    })
}

/// GraphQL playground UI
#[rustapi_rs::get("/graphql")]
async fn graphql_playground() -> Html<String> {
    Html(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>GraphQL Playground</title>
            <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/graphql-playground-react/build/static/css/index.css" />
            <script src="https://cdn.jsdelivr.net/npm/graphql-playground-react/build/static/js/middleware.js"></script>
        </head>
        <body>
            <div id="root"></div>
            <script>
                window.addEventListener('load', function() {
                    GraphQLPlayground.init(document.getElementById('root'), {
                        endpoint: '/graphql',
                        settings: {
                            'request.credentials': 'include'
                        }
                    })
                })
            </script>
        </body>
        </html>
        "#.to_string(),
    )
}

/// Index response
#[derive(Serialize, Schema)]
struct IndexResponse {
    message: String,
    endpoints: Endpoints,
    example_query: String,
}

#[derive(Serialize, Schema)]
struct Endpoints {
    graphql: String,
    playground: String,
}

/// Root endpoint
#[rustapi_rs::get("/")]
async fn index() -> Json<IndexResponse> {
    Json(IndexResponse {
        message: "GraphQL API Demo".to_string(),
        endpoints: Endpoints {
            graphql: "/graphql".to_string(),
            playground: "/graphql (GET)".to_string(),
        },
        example_query: r#"
{
  books {
    id
    title
    author
    year
  }
}
        "#.to_string(),
    })
}

// ============================================
// Main
// ============================================

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let db = Database::new();

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(db)
        .finish();

    println!("🚀 Starting GraphQL API Demo...");
    println!("📍 GraphQL Playground: http://127.0.0.1:8080/graphql");
    println!("\n📊 Example Query:");
    println!(
        r#"
{{
  books {{
    id
    title
    author
    year
  }}
}}
    "#
    );

    RustApi::auto()
        .state(schema)
        .run("127.0.0.1:8080")
        .await
}
