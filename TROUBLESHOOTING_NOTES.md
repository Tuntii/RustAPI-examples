# RustAPI Sorun Giderme Notları

Bu doküman, RustAPI ile çalışırken karşılaşılan yaygın sorunları ve çözümlerini içerir.

## ✅ Temel Kurallar

### 1. **`Schema` Derive Macro'su Kullanımı**

**Sorun:**
```rust
#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub page: Option<u32>,
}
```
```
error[E0277]: the trait bound `...: Handler<_>` is not satisfied
```

**Çözüm:**
Query parametreleri için kullanılan struct'lara **mutlaka** `Schema` derive macro'su eklenmelidir:

```rust
#[derive(Debug, Deserialize, Schema)]  // ✅ Schema eklendi
pub struct ListParams {
    pub page: Option<u32>,
}
```

**Neden?**
- RustAPI, OpenAPI dökümantasyonu oluşturmak için tüm extractorların schema bilgisine ihtiyaç duyar
- `Query<T>` extractor'ı, `T: RustApiSchema` trait bound'ı gerektirir
- `Schema` derive macro'su bu trait'i otomatik implement eder

---

### 2. **utoipa Değil, rustapi-openapi Kullan**

**Yanlış:**
```toml
[dependencies]
utoipa = "4.2"  # ❌ Kullanma
```

**Doğru:**
```toml
[dependencies]
rustapi-rs = { version = "0.1.233", features = ["full"] }
rustapi-openapi = "0.1.233"  # ✅ Bunu kullan
```

**Not:**
- RustAPI kendi OpenAPI implementasyonunu kullanır (`rustapi-openapi`)
- `utoipa` eklersen bağımlılık çakışmaları olabilir
- `Schema` derive macro'su `rustapi_rs::prelude::*` içinde zaten var

---

### 3. **rustapi_extras Değil, rustapi_rs Kullan**

**Sorun:**
```rust
use rustapi_extras::SqlxErrorExt;  // ❌ Eski isim
```
```
error[E0432]: unresolved import `rustapi_extras`
  --> src/main.rs:24:5
   |
24 | use rustapi_extras::SqlxErrorExt;
   |     ^^^^^^^^^^^^^^ use of unresolved module or unlinked crate `rustapi_extras`
```

**Çözüm:**
```rust
use rustapi_rs::SqlxErrorExt;  // ✅ Doğru import
```

**Neden?**
- `rustapi_extras` eski bir modül ismidir ve artık mevcut değildir
- SQLx error extension trait'i artık doğrudan `rustapi_rs` içindedir
- Bu trait'i kullanmak için `rustapi-rs`'nin `sqlx` feature'ını etkinleştirmeniz gerekir

**Gerekli Konfigürasyon:**
```toml
[dependencies]
rustapi-rs = { version = "0.1.233", features = ["sqlx"] }
```

**Kullanım:**
```rust
use rustapi_rs::prelude::*;
use rustapi_rs::SqlxErrorExt;  // ✅ Doğru path

async fn handler() -> Result<Json<Data>> {
    let data = sqlx::query_as::<_, Data>("SELECT * FROM items")
        .fetch_all(&pool)
        .await
        .map_err(|e| e.into_api_error())?;  // SqlxErrorExt trait metodu
    
    Ok(Json(data))
}
```

---

### 4. **rustapi_core ve rustapi_macros Kullanma**

**Sorun:**
```rust
use rustapi_core::{RustApi, health::HealthCheckBuilder};  // ❌ Eski modül
use rustapi_macros::get;  // ❌ Eski macro path
```
```
error[E0433]: failed to resolve: use of unresolved module or unlinked crate `rustapi_core`
error[E0433]: failed to resolve: use of unresolved module or unlinked crate `rustapi_macros`
```

**Çözüm:**
```rust
use rustapi_rs::prelude::*;  // ✅ Her şey burada
```

**Macro Kullanımı:**
```rust
// ❌ Eski (çalışmaz)
#[rustapi_macros::get("/")]
async fn index() -> &'static str { ... }

// ✅ Yeni (doğru)
#[rustapi_rs::get("/")]
async fn index() -> &'static str { ... }
```

**Route Kaydı:**
```rust
// ❌ Manuel mount (deprecated)
let app = RustApi::new()
    .mount(handler1)
    .mount(handler2);

// ✅ Auto-registration (önerilen)
RustApi::auto()  // Macro'lu handler'ları otomatik bulur
    .state(my_state)
    .layer(my_middleware)
    .run("127.0.0.1:3000")
    .await
```

**Neden?**
- `rustapi_core` ve `rustapi_macros` internal modüllerdir ve doğrudan import edilmemelidir
- Tüm public API `rustapi_rs` crate'inden export edilir
- `RustApi::auto()` kullanarak macro'lu handler'ları manuel kaydetmeye gerek kalmaz

---

### 5. **Query Extractor ile Attribute Macros**

**Yanlış:**
```rust
#[derive(Debug, Deserialize, IntoParams)]  // ❌ IntoParams utoipa'dan
pub struct ListParams {
    #[param(minimum = 1)]  // ❌ param attribute yok
    pub page: Option<u32>,
}
```

**Doğru:**
```rust
#[derive(Debug, Deserialize, Schema)]  // ✅ Schema kullan
pub struct ListParams {
    /// Page number (1-indexed)  // ✅ Doc comments OpenAPI'ye yansır
    pub page: Option<u32>,
}
```

**Notlar:**
- `#[param(...)]` attribute'u RustAPI'de yok
- Validation kuralları için `validator` crate kullan:
  ```rust
  #[derive(Debug, Deserialize, Validate, Schema)]
  pub struct CreateTask {
      #[validate(length(min = 1, max = 200))]
      pub title: String,
  }
  ```

---

### 5. **Handler Macro Kullanımı**

**Doğru Kullanım:**
```rust
#[rustapi_rs::get("/tasks")]
#[rustapi_rs::tag("Tasks")]
#[rustapi_rs::summary("List Tasks")]
async fn list_tasks(
    State(store): State<TaskStore>,
    Query(params): Query<ListParams>,  // ✅ Schema derive'lı struct
) -> Json<PaginatedTasks> {
    // ...
}
```

**RustApi::auto() ile Kullanım:**
```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Handler macro'ları kullanıldığında auto() yeterli
    RustApi::auto()
        .state(store)
        .run("127.0.0.1:8080")
        .await
}
```

---

### 5. **serde_json::Value ile Schema Sorunu**

**Sorun:**
```rust
async fn handler() -> Json<serde_json::Value> {  // ❌ Schema yok
    Json(json!({ "key": "value" }))
}
```
```
error: the trait `RustApiSchema` is not implemented for `serde_json::Value`
```

**Çözüm 1 - Wrapper Struct (Önerilen):**
```rust
#[derive(Serialize, Schema)]
struct MyResponse {
    key: String,
}

async fn handler() -> Json<MyResponse> {  // ✅ Type-safe
    Json(MyResponse {
        key: "value".to_string(),
    })
}
```

**Çözüm 2 - String olarak dönme:**
```rust
#[derive(Serialize, Schema)]
struct JsonResponse {
    data: String,  // JSON as string
}

async fn handler() -> Json<JsonResponse> {
    let value = json!({ "key": "value" });
    Json(JsonResponse {
        data: serde_json::to_string(&value).unwrap(),
    })
}
```

**Neden?**
- `serde_json::Value` RustAPI'nin Schema trait'ini implement etmez
- OpenAPI dökümantasyonu için concrete type'lar gerekir
- Type-safe struct'lar hata yakalamayı kolaylaştırır

---

### 6. **impl IntoResponse Dönüş Tipi**

**Sorun:**
```rust
#[rustapi_rs::get("/")]
async fn handler() -> impl IntoResponse {  // ❌ Handler trait error
    Html("<h1>Hello</h1>")
}
```

**Çözüm:**
Concrete type kullan:
```rust
#[rustapi_rs::get("/")]
async fn handler() -> Html<String> {  // ✅ Concrete type
    Html("<h1>Hello</h1>".to_string())
}
```

**Alternatif Response Types:**
- `Html<String>` - HTML içerik
- `Json<T>` - JSON response (T: Schema olmalı)
- `String` - Plain text
- `StatusCode` - Sadece status code
- `(StatusCode, Json<T>)` - Status + JSON

---

## 📋 Checklist: Yeni Bir Handler Eklerken

- [ ] Query params struct'ına `Schema` derive ekle
- [ ] Response struct'larına `Schema` derive ekle
- [ ] Request body struct'larına `Schema` derive ekle
- [ ] Validation gerekiyorsa `Validate` derive ve attribute'lar ekle
- [ ] `#[rustapi_rs::get/post/...]` macro'larını kullan
- [ ] `RustApi::auto()` ile automatic route discovery kullan
- [ ] `cargo check` ile compile et
- [ ] Swagger UI'dan test et (`http://127.0.0.1:8080/docs`)

---

## 🔍 Debug İpuçları

### Hata: "Handler trait not implemented"

1. **Tüm extractor'ların Schema implement ettiğinden emin ol:**
   ```rust
   Query(params): Query<ListParams>  // ListParams: Schema olmalı
   ```

2. **FromRequest trait'i doğru implement edilmiş mi kontrol et:**
   - `State<T>`, `Query<T>`, `Path<T>` → `FromRequestParts` implement eder
   - `Json<T>`, `Body` → `FromRequest` implement eder
   - `FromRequestParts` otomatik olarak `FromRequest` implement eder (blanket impl)

3. **Parametre sırası önemli:**
   ```rust
   async fn handler(
       State(...): State<...>,     // ✅ State first
       Query(...): Query<...>,      // ✅ Query params
       Json(...): Json<...>,        // ✅ Body son olmalı
   ) -> ... 
   ```

### Hata: "State not found"

```rust
// State'i ekle:
RustApi::auto()
    .state(my_state)  // ← Bunu unutma!
    .run("0.0.0.0:8080")
    .await
```

---

## 📚 Faydalı Kaynaklar

- [RustAPI Cookbook](https://tuntii.github.io/RustAPI/cookbook/)
- [Examples Repository](https://github.com/Tuntii/rustapi-rs-examples)
- [RustAPI Documentation](https://docs.rs/rustapi-rs/)

---

## 🎯 Özet

**En önemli 3 kural:**
1. **Query/Path/Response struct'larına `Schema` derive ekle**
2. **`utoipa` kullanma, `rustapi-openapi` zaten var**
3. **`RustApi::auto()` kullanırken handler macro'larını kullan**

Bu kurallara uyarsan RustAPI ile sorunsuz çalışırsın! 🚀
