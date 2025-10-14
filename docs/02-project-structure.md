# Project Structure

Understanding the codebase organization and file layout.

## 📁 Directory Structure

```plaintext
rust-axum-backend/
├── Cargo.toml              # Project metadata and dependencies
├── README.md               # Project overview
├── src/                    # Source code
│   ├── main.rs            # Application entry point
│   ├── error.rs           # Error types and handling
│   ├── ctx.rs             # User context (session)
│   ├── model.rs           # Data models and business logic
│   ├── log.rs             # Request logging
│   └── web/               # Web layer
│       ├── mod.rs         # Web module declaration
│       ├── mw_auth.rs     # Authentication middleware
│       ├── routes_login.rs # Login endpoints
│       └── routes_ticket.rs # Ticket CRUD endpoints
├── tests/                  # Integration tests
│   └── quick_dev.rs       # Quick development tests
├── docs/                   # Documentation (this directory)
└── target/                 # Build artifacts (generated)
```

## 📄 File Responsibilities

### **`src/main.rs`** - Application Entry Point

**Purpose:** Bootstrap the application, configure routes, and start the server.

**Key components:**

- `#[tokio::main] async fn main()` - Async runtime entry
- Router configuration - Merge routes and apply layers
- Server initialization - Bind to port and serve

**Imports from:**

- `error.rs` - Error and Result types
- `model.rs` - ModelController
- `ctx.rs` - Ctx type
- `web/*` - Route handlers and middleware

**Pattern:**

```rust
// 1. Initialize shared state
let mc = ModelController::new().await?;

// 2. Build route trees
let routes_apis = web::routes_ticket::routes(mc.clone())
    .route_layer(auth_middleware);

// 3. Compose application
let app = Router::new()
    .merge(routes)
    .layer(middleware);

// 4. Start server
axum::serve(listener, app).await?;
```

---

### **`src/error.rs`** - Error Handling

**Purpose:** Define all application errors and their HTTP representations.

**Key components:**

```rust
// Application-specific error type
pub enum Error {
    LoginFail,
    AuthFailNoAuthTokenCookie,
    TicketDeleteFailIdNotFound { id: u64 },
}

// Convenient Result alias
pub type Result<T> = core::result::Result<T, Error>;

// HTTP response conversion
impl IntoResponse for Error { ... }

// Client-safe error format
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    SERVICE_ERROR,
}
```

**Pattern:** Centralized error types with automatic HTTP conversion.

---

### **`src/ctx.rs`** - User Context

**Purpose:** Store authenticated user information for request duration.

**Key components:**

```rust
#[derive(Clone, Debug)]
pub struct Ctx {
    user_id: u64,
}

impl Ctx {
    pub fn new(user_id: u64) -> Self { ... }
    pub fn user_id(&self) -> u64 { ... }
}

// Custom extractor for handlers
impl<S> FromRequestParts<S> for Ctx { ... }
```

**Lifecycle:**

1. Created by `mw_ctx_resolver` middleware
2. Stored in `request.extensions()`
3. Extracted by handlers via `FromRequestParts`
4. Dropped when request completes

**Pattern:** Request-scoped data via extensions + custom extractor.

---

### **`src/model.rs`** - Data Layer

**Purpose:** Business logic and data persistence (currently in-memory).

**Key components:**

```rust
// Shared state for concurrent access
#[derive(Clone)]
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

// Data models
#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub title: String,
}

// Input models (for creation)
#[derive(Deserialize)]
pub struct TicketForCreate {
    pub title: String,
}

// CRUD operations
impl ModelController {
    pub async fn create_ticket(...) -> Result<Ticket> { ... }
    pub async fn list_tickets(...) -> Result<Vec<Ticket>> { ... }
    pub async fn delete_ticket(...) -> Result<Ticket> { ... }
}
```

**Pattern:** Repository pattern with in-memory storage.

---

### **`src/log.rs`** - Request Logging

**Purpose:** Log all requests with UUID tracking for debugging.

**Key components:**

```rust
pub async fn log_request(
    uuid: Uuid,
    req_method: Method,
    uri: Uri,
    ctx: Option<Ctx>,
    service_error: Option<&Error>,
    client_error: Option<&ClientError>,
) -> Result<()> {
    // Log request details to console/file
}
```

**When called:** By `main_response_mapper` after every request.

**Pattern:** Structured logging with correlation IDs.

---

### **`src/web/mod.rs`** - Web Module Declaration

**Purpose:** Export web layer submodules.

```rust
pub mod mw_auth;
pub mod routes_login;
pub mod routes_ticket;
```

**Pattern:** Module organization - keeps web layer separate from business logic.

---

### **`src/web/mw_auth.rs`** - Authentication Middleware

**Purpose:** Extract and validate authentication tokens, create user context.

**Key components:**

```rust
// Middleware to resolve user context from cookies
pub async fn mw_ctx_resolver(
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response> {
    // 1. Extract auth token from cookie
    // 2. Parse and validate token
    // 3. Create Ctx
    // 4. Store in request.extensions()
    // 5. Continue to next layer
}

// Middleware to require authentication
pub async fn mw_require_auth(
    ctx: Result<Ctx>,
    req: Request<Body>,
    next: Next,
) -> Result<Response> {
    // Return 403 if Ctx not present
    ctx?;
    Ok(next.run(req).await)
}

// Helper function to parse token
fn parse_token(token: String) -> Result<(u64, String)> {
    // Parse "user-{id}.{signature}" format
}

// Custom extractor for Ctx
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    async fn from_request_parts(...) -> Result<Self> {
        // Extract from request.extensions()
    }
}
```

**Pattern:** Two-phase auth - resolve context, then require it on protected routes.

---

### **`src/web/routes_login.rs`** - Login Endpoints

**Purpose:** Handle user authentication and cookie management.

**Key components:**

```rust
pub fn routes() -> Router {
    Router::new().route("/api/login", get(api_login))
}

async fn api_login(
    Query(login): Query<LoginPayload>,
    cookies: Cookies,
) -> Result<Json<Value>> {
    // 1. Validate credentials (hardcoded for demo)
    // 2. Generate auth token
    // 3. Set cookie
    // 4. Return success
}
```

**Pattern:** Simple query-based login (replace with POST + body in production).

---

### **`src/web/routes_ticket.rs`** - Ticket CRUD

**Purpose:** REST API for ticket management.

**Key components:**

```rust
pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(mc)
}

#[axum::debug_handler]
async fn create_ticket(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    let ticket = mc.create_ticket(ctx, ticket_fc).await?;
    Ok(Json(ticket))
}

// Similar handlers for list_tickets and delete_ticket
```

**Pattern:** RESTful routes with extractor-based dependency injection.

---

### **`tests/quick_dev.rs`** - Integration Tests

**Purpose:** Quick HTTP tests for development.

**Key components:**

```rust
#[tokio::test]
async fn quick_dev() -> anyhow::Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    // Test login
    hc.do_get("/api/login?username=demo&pwd=welcome").await?;

    // Test CRUD operations
    let create_res = hc.do_post("/api/tickets", json!({...})).await?;
    let list_res = hc.do_get("/api/tickets").await?;
    let delete_res = hc.do_delete("/api/tickets/0").await?;

    Ok(())
}
```

**Usage:** `cargo test`

**Pattern:** End-to-end testing with real HTTP requests.

---

## 🔄 Data Flow Between Files

```plaintext
main.rs
  └─> Initializes ModelController (model.rs)
  └─> Registers middleware (web/mw_auth.rs)
  └─> Registers routes (web/routes_*.rs)
  └─> Applies response mapper (uses error.rs, log.rs)

Request comes in:
  1. CookieManagerLayer (tower-cookies)
  2. mw_ctx_resolver (web/mw_auth.rs)
     └─> Creates Ctx (ctx.rs)
  3. Route handler (web/routes_ticket.rs)
     └─> Calls ModelController (model.rs)
     └─> May return Error (error.rs)
  4. main_response_mapper (main.rs)
     └─> Logs request (log.rs)
     └─> Converts errors (error.rs)

Response goes out
```

## 🎨 Design Patterns by File

| File              | Pattern            | Purpose                           |
| ----------------- | ------------------ | --------------------------------- |
| `main.rs`         | Composition Root   | Assemble application components   |
| `error.rs`        | Centralized Errors | Single source of truth for errors |
| `ctx.rs`          | Request Context    | Pass user data through middleware |
| `model.rs`        | Repository         | Abstract data access              |
| `log.rs`          | Structured Logging | Consistent log format             |
| `web/mw_auth.rs`  | Middleware Chain   | Composable request processing     |
| `web/routes_*.rs` | REST API           | Resource-based endpoints          |

## 🔧 Module System Explanation

### How `mod` Works

```rust
// In main.rs
mod web;  // Tells Rust to look for:
          // 1. web.rs, OR
          // 2. web/mod.rs

// In web/mod.rs
pub mod mw_auth;  // Tells Rust to look for:
                  // 1. web/mw_auth.rs, OR
                  // 2. web/mw_auth/mod.rs
```

### Visibility Rules

```rust
// Private (default) - only accessible within module
mod internal;
fn private_fn() {}

// Public - accessible from parent modules
pub mod public;
pub fn public_fn() {}

// Public within crate - accessible anywhere in this crate
pub(crate) fn crate_fn() {}
```

### Using Re-exports

```rust
// In main.rs
pub use self::error::{Error, Result};

// Now other files can:
use crate::{Error, Result};  // Short path

// Instead of:
use crate::error::{Error, Result};  // Long path
```

## 📦 Dependencies Organization

### Production Dependencies (`Cargo.toml`)

```toml
[dependencies]
# Web framework
axum = { version = "0.8.6", features = ["macros"] }

# Async runtime
tokio = { version = "1.47.1", features = ["full"] }

# Serialization
serde = { version = "1.0.228", features = ["derive"] }
serde_json = "1.0.145"

# Middleware
tower-cookies = "0.11.0"
tower-http = { version = "0.6.6", features = ["fs"] }

# Utilities
uuid = { version = "1.18.1", features = ["v4", "fast-rng"] }
lazy-regex = "3.4.1"
```

### Development Dependencies

```toml
[dev-dependencies]
anyhow = "1.0.100"       # Simplified error handling in tests
httpc-test = "0.1.10"    # HTTP testing client
```

## 🎯 Adding New Features

### Adding a New Route

1. **Create handler in `web/routes_*.rs`:**

   ```rust
   async fn my_handler(
       State(mc): State<ModelController>,
       ctx: Ctx,
   ) -> Result<Json<MyData>> {
       // Implementation
   }
   ```

2. **Register route:**

   ```rust
   pub fn routes(mc: ModelController) -> Router {
       Router::new()
           .route("/my-route", get(my_handler))
           .with_state(mc)
   }
   ```

3. **Merge in `main.rs`:**

   ```rust
   let routes_all = Router::new()
       .merge(web::my_routes::routes(mc.clone()))
       // ...
   ```

### Adding a New Model

1. **Define in `model.rs`:**

   ```rust
   #[derive(Clone, Debug, Serialize, Deserialize)]
   pub struct MyModel {
       pub id: u64,
       pub name: String,
   }
   ```

2. **Add storage to ModelController:**

   ```rust
   pub struct ModelController {
       tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
       my_models_store: Arc<Mutex<Vec<Option<MyModel>>>>,  // New!
   }
   ```

3. **Implement CRUD methods:**

   ```rust
   impl ModelController {
       pub async fn create_my_model(&self, ...) -> Result<MyModel> {
           // Implementation
       }
   }
   ```

### Adding a New Error

1. **Add variant to `error.rs`:**

   ```rust
   pub enum Error {
       // Existing errors...
       MyNewError { details: String },
   }
   ```

2. **Add client mapping:**

   ```rust
   impl Error {
       pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
           match self {
               // ...
               Self::MyNewError { .. } => (
                   StatusCode::BAD_REQUEST,
                   ClientError::INVALID_PARAMS
               ),
           }
       }
   }
   ```

## ✅ Key Takeaways

1. **Separation of concerns** - Each file has a single responsibility
2. **Module privacy** - Use `pub` only when necessary
3. **Type safety** - Models and errors are strongly typed
4. **Middleware composition** - Auth, logging, etc. as separate layers
5. **Testability** - Integration tests exercise full HTTP stack

## 📚 Next Steps

- [Rust Fundamentals](./03-rust-fundamentals.md) - Core language concepts
- [Middleware & Layers](./06-middleware-layers.md) - Deep dive into web layer
- [Common Patterns](./12-common-patterns.md) - Reusable code patterns
