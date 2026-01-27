# Templates Example

Server-side HTML rendering with Tera templates in RustAPI.

> 📖 **Cookbook**: [Crates → rustapi-view](https://tuntii.github.io/RustAPI/)

## Overview

This example demonstrates server-side rendering with:

- Tera template engine integration
- Template inheritance (layouts)
- Type-safe context building
- Static file serving
- Form handling

## Prerequisites

- Rust 1.70+
- Basic HTML/CSS knowledge
- Understanding of template engines

## Features Demonstrated

| Feature | Description |
|---------|-------------|
| `View<T>` | Template response type |
| `Templates` | Tera engine wrapper |
| Template inheritance | Base layouts |
| `ContextBuilder` | Type-safe context |
| Static files | CSS/JS serving |

## Quick Start

```bash
# Run the example
cargo run -p templates-example

# Server starts at http://127.0.0.1:8080
# Open in browser to see rendered HTML
```

## Routes

| Method | Path | Description |
|--------|------|-------------|
| GET | `/` | Home page with features |
| GET | `/about` | About page |
| GET | `/contact` | Contact form |
| POST | `/contact` | Form submission |
| GET | `/blog` | Blog listing |
| GET | `/static/*` | Static files (CSS) |

## Project Structure

```
templates/
├── src/
│   └── main.rs
├── templates/
│   ├── base.html      # Base layout
│   ├── index.html     # Home page
│   ├── about.html     # About page
│   ├── contact.html   # Contact form
│   └── blog.html      # Blog listing
└── static/
    └── style.css      # Stylesheet
```

## Testing

### View Pages

```bash
# Home page
curl http://127.0.0.1:8080/

# About page
curl http://127.0.0.1:8080/about

# Blog page
curl http://127.0.0.1:8080/blog

# Or open in browser
open http://127.0.0.1:8080/
```

### Submit Contact Form

```bash
curl -X POST "http://127.0.0.1:8080/contact?name=Alice&message=Hello"
```

## Code Walkthrough

### 1. Template Context Types

```rust
#[derive(Serialize)]
struct HomeContext {
    title: String,
    features: Vec<Feature>,
}

#[derive(Serialize)]
struct Feature {
    name: String,
    description: String,
}
```

### 2. View Handler

```rust
use rustapi_rs::view::{Templates, View};

async fn home(State(templates): State<Templates>) -> View<HomeContext> {
    let features = vec![
        Feature {
            name: "Type-Safe".to_string(),
            description: "Compile-time validation".to_string(),
        },
        Feature {
            name: "Fast".to_string(),
            description: "Built on Tokio and Hyper".to_string(),
        },
    ];

    View::render(
        &templates,
        "index.html",
        HomeContext {
            title: "Home".to_string(),
            features,
        },
    )
    .await
}
```

### 3. Form Handling

```rust
#[derive(Debug, Deserialize)]
struct ContactForm {
    name: Option<String>,
    message: Option<String>,
}

async fn contact_post(
    State(templates): State<Templates>,
    Query(params): Query<ContactForm>,
) -> View<ContactContext> {
    tracing::info!("Contact form submitted: {:?}", params);

    View::render(
        &templates,
        "contact.html",
        ContactContext {
            title: "Contact".to_string(),
            submitted: true,
            name: params.name,
            message: params.message,
        },
    )
    .await
}
```

### 4. Server Setup

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize templates from directory
    let templates = Templates::new("templates")?;
    
    RustApi::new()
        .route("/", get(home))
        .route("/about", get(about))
        .route("/contact", get(contact_get).post(contact_post))
        .route("/blog", get(blog))
        // Serve static files
        .static_files("/static", "static")
        .state(templates)
        .run("127.0.0.1:8080")
        .await
}
```

## Template Examples

### Base Layout (base.html)

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{ title }} - RustAPI</title>
    <link rel="stylesheet" href="/static/style.css">
</head>
<body>
    <nav>
        <a href="/">Home</a>
        <a href="/about">About</a>
        <a href="/blog">Blog</a>
        <a href="/contact">Contact</a>
    </nav>
    
    <main>
        {% block content %}{% endblock %}
    </main>
    
    <footer>
        <p>Powered by RustAPI</p>
    </footer>
</body>
</html>
```

### Page Template (index.html)

```html
{% extends "base.html" %}

{% block content %}
<h1>Welcome to RustAPI</h1>

<section class="features">
    {% for feature in features %}
    <div class="feature">
        <h3>{{ feature.name }}</h3>
        <p>{{ feature.description }}</p>
    </div>
    {% endfor %}
</section>
{% endblock %}
```

### Conditional Rendering

```html
{% if submitted %}
    <div class="success">
        <p>Thank you, {{ name | default(value="Anonymous") }}!</p>
        <p>Your message: {{ message }}</p>
    </div>
{% else %}
    <form method="post">
        <input type="text" name="name" placeholder="Your name">
        <textarea name="message" placeholder="Your message"></textarea>
        <button type="submit">Send</button>
    </form>
{% endif %}
```

### Loops and Filters

```html
{% for post in posts %}
<article>
    <h2>{{ post.title }}</h2>
    <p class="meta">By {{ post.author }} on {{ post.date }}</p>
    <p>{{ post.excerpt | truncate(length=100) }}</p>
    <a href="/blog/{{ post.id }}">Read more →</a>
</article>
{% endfor %}
```

## Key Concepts

### Template Inheritance

```
base.html
    └── index.html    (extends base)
    └── about.html    (extends base)
    └── contact.html  (extends base)
```

### Tera Features

| Feature | Syntax |
|---------|--------|
| Variables | `{{ variable }}` |
| Blocks | `{% block name %}{% endblock %}` |
| Extends | `{% extends "base.html" %}` |
| Loops | `{% for item in items %}{% endfor %}` |
| Conditions | `{% if condition %}{% endif %}` |
| Filters | `{{ text \| upper }}` |
| Include | `{% include "partial.html" %}` |

### Static Files

```rust
// Serve entire directory
.static_files("/static", "static")

// In templates:
<link href="/static/style.css" rel="stylesheet">
<script src="/static/app.js"></script>
<img src="/static/images/logo.png">
```

## Cargo.toml

```toml
[dependencies]
rustapi-rs = { version = "0.2", features = ["view"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
utoipa = "4"
tracing = "0.1"
```

## Common Patterns

### Partial Templates

```html
<!-- partials/header.html -->
<header>
    <h1>{{ site_name }}</h1>
</header>

<!-- page.html -->
{% include "partials/header.html" %}
```

### Custom Filters

```rust
let mut templates = Templates::new("templates")?;
templates.register_filter("markdown", |value, _| {
    // Convert markdown to HTML
    Ok(markdown_to_html(value.as_str().unwrap()))
});
```

### Error Pages

```rust
async fn not_found(State(templates): State<Templates>) -> View<ErrorContext> {
    View::render(&templates, "404.html", ErrorContext {
        title: "Not Found".to_string(),
        message: "Page not found".to_string(),
    })
    .await
    .with_status(StatusCode::NOT_FOUND)
}
```

## Next Steps

- **[websocket](../websocket/)** — Real-time features
- **[proof-of-concept](../proof-of-concept/)** — Full application

## Related Documentation

- [FEATURES.md](../FEATURES.md#view) — View feature reference
- [Tera Documentation](https://tera.netlify.app/)
- [RustAPI Cookbook](https://tuntii.github.io/RustAPI/)

---

<div align="center">

**[← Back to Examples](../README.md)**

</div>
