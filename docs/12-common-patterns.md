# Common Patterns & Quick Reference

Reusable patterns and code snippets found in this codebase.

## 🎯 Handler Patterns

### Basic Handler

```rust
use axum::{Json, response::Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CreateInput {
    name: String,
}

#[derive(Serialize)]
struct Output {
    id: u64,
    name: String,
}

async fn create_handler(
    Json(input): Json<CreateInput>,
) -> Result<Json<Output>> {
    let output = Output {
        id: 1,
        name: input.name,
    };
    Ok(Json(output))
}
```

### Handler with State

```rust
use axum::extract::State;

async fn handler_with_state(
    State(mc): State<ModelController>,
    Json(input): Json<CreateInput>,
) -> Result<Json<Output>> {
    let result = mc.create(input).await?;
    Ok(Json(result))
}
```

### Handler with Auth

```rust
use crate::ctx::Ctx;

async fn protected_handler(
    State(mc): State<ModelController>,
    ctx: Ctx,  // Automatically extracted, fails if not authenticated
    Json(input): Json<CreateInput>,
) -> Result<Json<Output>> {
    let user_id = ctx.user_id();
    let result = mc.create_for_user(user_id, input).await?;
    Ok(Json(result))
}
```

### Handler with Path Parameter

```rust
use axum::extract::Path;

async fn get_by_id(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Path(id): Path<u64>,
) -> Result<Json<Item>> {
    let item = mc.get(ctx, id).await?;
    Ok(Json(item))
}
```

### Handler with Query Parameters

```rust
use axum::extract::Query;
use serde::Deserialize;

#[derive(Deserialize)]
struct Pagination {
    page: Option<u32>,
    limit: Option<u32>,
}

async fn list_with_pagination(
    Query(params): Query<Pagination>,
) -> Result<Json<Vec<Item>>> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);
    // ...
}
```

## 🛣️ Routing Patterns

### Basic Routes

```rust
use axum::{Router, routing::{get, post, delete}};

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/items", post(create_item).get(list_items))
        .route("/items/:id", get(get_item).delete(delete_item))
        .with_state(mc)
}
```

### Nested Routes

```rust
pub fn routes(mc: ModelController) -> Router {
    let api_routes = Router::new()
        .route("/users", get(list_users))
        .route("/posts", get(list_posts));

    Router::new()
        .nest("/api/v1", api_routes)
        .with_state(mc)
}
```

### Protected Routes

```rust
pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .route_layer(middleware::from_fn(mw_require_auth))  // Auth required
        .with_state(mc)
}
```

## 🧅 Middleware Patterns

### Request Logger

```rust
async fn log_request(
    uri: Uri,
    method: Method,
    req: Request<Body>,
    next: Next,
) -> Result<Response> {
    println!("→ {} {}", method, uri);
    Ok(next.run(req).await)
}
```

### Response Logger

```rust
async fn log_response(
    req: Request<Body>,
    next: Next,
) -> Result<Response> {
    let res = next.run(req).await;
    println!("← {}", res.status());
    Ok(res)
}
```

### Error Handler

```rust
async fn handle_errors(
    req: Request<Body>,
    next: Next,
) -> Result<Response> {
    let res = next.run(req).await;

    if res.status().is_client_error() {
        // Log client errors
        eprintln!("Client error: {}", res.status());
    }

    Ok(res)
}
```

### Request ID Generator

```rust
use uuid::Uuid;

async fn add_request_id(
    mut req: Request<Body>,
    next: Next,
) -> Result<Response> {
    let request_id = Uuid::new_v4();
    req.extensions_mut().insert(request_id);
    Ok(next.run(req).await)
}
```

## 🎭 Error Patterns

### Return Error

```rust
async fn operation() -> Result<Data> {
    if condition {
        return Err(Error::SomeError);
    }
    Ok(data)
}
```

### Propagate Error with `?`

```rust
async fn operation() -> Result<Data> {
    let step1 = fallible_step1().await?;
    let step2 = fallible_step2(step1)?;
    Ok(step2)
}
```

### Convert Option to Result

```rust
async fn find_item(id: u64) -> Result<Item> {
    items
        .get(id)
        .ok_or(Error::NotFound { id })?
}
```

### Transform Error

```rust
async fn operation() -> Result<Data> {
    external_call()
        .await
        .map_err(|e| Error::ExternalError(e.to_string()))?
}
```

### Add Context to Error

```rust
async fn load_config() -> Result<Config> {
    std::fs::read_to_string("config.json")
        .map_err(|e| Error::ConfigError(format!("Failed to read config: {}", e)))?
}
```

## 🗄️ Model Patterns

### CRUD Model

```rust
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct ModelController {
    store: Arc<Mutex<Vec<Item>>>,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            store: Arc::new(Mutex::new(Vec::new())),
        })
    }

    pub async fn create(&self, ctx: Ctx, data: CreateInput) -> Result<Item> {
        let mut store = self.store.lock().unwrap();
        let item = Item {
            id: store.len() as u64,
            user_id: ctx.user_id(),
            data,
        };
        store.push(item.clone());
        Ok(item)
    }

    pub async fn list(&self, ctx: Ctx) -> Result<Vec<Item>> {
        let store = self.store.lock().unwrap();
        Ok(store.clone())
    }

    pub async fn get(&self, ctx: Ctx, id: u64) -> Result<Item> {
        let store = self.store.lock().unwrap();
        store
            .get(id as usize)
            .cloned()
            .ok_or(Error::NotFound { id })
    }

    pub async fn delete(&self, ctx: Ctx, id: u64) -> Result<Item> {
        let mut store = self.store.lock().unwrap();
        if (id as usize) < store.len() {
            Ok(store.remove(id as usize))
        } else {
            Err(Error::NotFound { id })
        }
    }
}
```

## 🔐 Authentication Patterns

### Extract User Context

```rust
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self> {
        parts
            .extensions
            .get::<Ctx>()
            .cloned()
            .ok_or(Error::AuthRequired)
    }
}
```

### Set Auth Cookie

```rust
use tower_cookies::{Cookie, Cookies};

async fn login(
    cookies: Cookies,
    Json(login): Json<LoginPayload>,
) -> Result<Json<Response>> {
    // Validate credentials
    let user = validate_user(login.username, login.password)?;

    // Create token
    let token = format!("user-{}.signature", user.id);

    // Set cookie
    cookies.add(Cookie::new("auth-token", token));

    Ok(Json(Response { success: true }))
}
```

### Clear Auth Cookie

```rust
async fn logout(cookies: Cookies) -> Result<Json<Response>> {
    cookies.remove(Cookie::named("auth-token"));
    Ok(Json(Response { success: true }))
}
```

## 🔄 Async Patterns

### Parallel Execution

```rust
use tokio::try_join;

async fn fetch_all_data() -> Result<(Users, Posts, Comments)> {
    let (users, posts, comments) = try_join!(
        fetch_users(),
        fetch_posts(),
        fetch_comments(),
    )?;

    Ok((users, posts, comments))
}
```

### Sequential with Error Handling

```rust
async fn process_pipeline() -> Result<Output> {
    let data = fetch_data().await?;
    let processed = process_data(data).await?;
    let validated = validate_data(processed).await?;
    Ok(validated)
}
```

### Spawn Background Task

```rust
use tokio::task;

async fn handler() -> Result<Json<Response>> {
    // Spawn background task
    task::spawn(async {
        if let Err(e) = send_email().await {
            eprintln!("Failed to send email: {}", e);
        }
    });

    // Return immediately
    Ok(Json(Response { success: true }))
}
```

## 📝 Serialization Patterns

### Custom Serialization

```rust
use serde::{Serialize, Serializer};
use chrono::{DateTime, Utc};

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,

    #[serde(serialize_with = "serialize_datetime")]
    created_at: DateTime<Utc>,
}

fn serialize_datetime<S>(
    dt: &DateTime<Utc>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&dt.to_rfc3339())
}
```

### Skip Serialization

```rust
#[derive(Serialize)]
struct User {
    pub id: u64,
    pub username: String,

    #[serde(skip)]
    pub password_hash: String,  // Never serialized
}
```

### Rename Fields

```rust
#[derive(Serialize, Deserialize)]
struct User {
    #[serde(rename = "userId")]
    pub user_id: u64,

    #[serde(rename = "firstName")]
    pub first_name: String,
}
```

## 🧪 Testing Patterns

### Integration Test

```rust
#[tokio::test]
async fn test_create_ticket() -> anyhow::Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    // Login
    hc.do_get("/api/login?username=demo&pwd=welcome").await?;

    // Create ticket
    let res = hc
        .do_post(
            "/api/tickets",
            json!({"title": "Test ticket"}),
        )
        .await?;

    let ticket: Ticket = res.json_body()?;
    assert_eq!(ticket.title, "Test ticket");

    Ok(())
}
```

### Unit Test

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_model_create() {
        let mc = ModelController::new().await.unwrap();
        let ctx = Ctx::new(1);

        let input = TicketForCreate {
            title: "Test".to_string(),
        };

        let ticket = mc.create_ticket(ctx, input).await.unwrap();
        assert_eq!(ticket.title, "Test");
    }
}
```

## 🎨 Type Patterns

### Type Alias

```rust
pub type Result<T> = core::result::Result<T, Error>;
pub type UserId = u64;
```

### Newtype Pattern

```rust
#[derive(Debug, Clone, Copy)]
pub struct UserId(pub u64);

impl UserId {
    pub fn new(id: u64) -> Self {
        UserId(id)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}
```

### Builder Pattern

```rust
pub struct QueryBuilder {
    filters: Vec<String>,
    limit: Option<u32>,
    offset: Option<u32>,
}

impl QueryBuilder {
    pub fn new() -> Self {
        Self {
            filters: Vec::new(),
            limit: None,
            offset: None,
        }
    }

    pub fn filter(mut self, condition: String) -> Self {
        self.filters.push(condition);
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn build(self) -> Query {
        Query {
            filters: self.filters,
            limit: self.limit.unwrap_or(10),
            offset: self.offset.unwrap_or(0),
        }
    }
}

// Usage
let query = QueryBuilder::new()
    .filter("status = 'active'".to_string())
    .limit(20)
    .offset(10)
    .build();
```

## 🔍 Debugging Patterns

### Debug Print

```rust
// Simple debug
println!("{:?}", value);

// Pretty debug
println!("{:#?}", value);

// Debug macro (returns value)
let result = dbg!(some_operation());
```

### Conditional Compilation

```rust
#[cfg(debug_assertions)]
fn debug_info() {
    println!("Debug mode");
}

#[cfg(not(debug_assertions))]
fn debug_info() {
    // No-op in release mode
}
```

### Logging Levels

```rust
// Different log levels
println!("INFO: {}", message);
eprintln!("ERROR: {}", error);

#[cfg(debug_assertions)]
println!("DEBUG: {}", debug_info);
```

## ✅ Quick Reference

### Common Extractors

```rust
State(state)               // Shared application state
ctx: Ctx                   // User context
Json(body)                 // JSON request body
Path(id)                   // URL path parameter
Query(params)              // Query string parameters
cookies: Cookies           // Cookie jar
uri: Uri                   // Request URI
method: Method             // HTTP method
headers: HeaderMap         // Request headers
```

### Common Return Types

```rust
Result<Json<T>>           // JSON response with error handling
Result<Html<String>>      // HTML response
Result<StatusCode>        // Status code only
Result<()>                // No body (204 No Content)
impl IntoResponse         // Any type implementing IntoResponse
```

### Common Attributes

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(skip)]
#[serde(skip_serializing_if = "Option::is_none")]
#[axum::debug_handler]
#[tokio::test]
#[cfg(test)]
```

## 📚 Next Steps

- [Architecture Overview](./01-architecture-overview.md) - System design
- [Error Handling](./05-error-handling.md) - Advanced error patterns
- [Middleware & Layers](./06-middleware-layers.md) - Layer system
- [Testing Guide](./14-testing-guide.md) - Testing strategies
