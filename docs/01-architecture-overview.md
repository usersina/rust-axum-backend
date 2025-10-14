# Architecture Overview

This document provides a high-level overview of the application architecture and request flow.

## 🏗️ System Architecture

```bash
┌─────────────────────────────────────────────────────────┐
│                        Client                           │
└────────────────────┬────────────────────────────────────┘
                     │ HTTP Request
                     ↓
┌─────────────────────────────────────────────────────────┐
│                   Layer Stack                           │
│  ┌───────────────────────────────────────────────────┐  │
│  │ CookieManagerLayer (Parse cookies)                │  │
│  │  ┌─────────────────────────────────────────────┐  │  │
│  │  │ mw_ctx_resolver (Extract user context)      │  │  │
│  │  │  ┌───────────────────────────────────────┐  │  │  │
│  │  │  │ Routes (Handle request)               │  │  │  │
│  │  │  │  ┌─────────────────────────────────┐  │  │  │  │
│  │  │  │  │ mw_require_auth (Protected)     │  │  │  │  │
│  │  │  │  │  ┌───────────────────────────┐  │  │  │  │  │
│  │  │  │  │  │ Handler (Business logic)  │  │  │  │  │  │
│  │  │  │  │  └───────────────────────────┘  │  │  │  │  │
│  │  │  │  └─────────────────────────────────┘  │  │  │  │
│  │  │  └───────────────────────────────────────┘  │  │  │
│  │  │  main_response_mapper (Transform errors)    │  │  │
│  │  └─────────────────────────────────────────────┘  │  │
│  └───────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
                     │ HTTP Response
                     ↓
┌─────────────────────────────────────────────────────────┐
│                        Client                           │
└─────────────────────────────────────────────────────────┘
```

## 🔄 Request Flow

### Example: Creating a Ticket

```txt
1. Client → POST /api/tickets
   Body: { "title": "Fix bug" }
   Cookie: auth-token=user-123.abc

2. CookieManagerLayer
   → Parses cookies from request
   → Makes them available to handlers

3. mw_ctx_resolver
   → Extracts auth-token cookie
   → Validates token format (user-{id}.{signature})
   → Creates Ctx object with user_id
   → Stores Ctx in request.extensions()

4. mw_require_auth (route_layer on /api/*)
   → Attempts to extract Ctx from request
   → Returns 403 if Ctx not present
   → Allows request to continue if authenticated

5. create_ticket Handler
   → Extracts: State(ModelController), Ctx, Json(body)
   → Calls mc.create_ticket(ctx, ticket_fc)
   → Returns: Ok(Json(ticket)) or Err(Error)

6. Error::into_response (if error occurred)
   → Converts internal Error to Response
   → Stores error in response.extensions()

7. main_response_mapper
   → Extracts error from response.extensions()
   → Converts to client-safe error format
   → Logs request with UUID
   → Returns sanitized response

8. Client ← 201 Created
   Body: { "id": 1, "title": "Fix bug" }
```

## 🧱 Component Layers

### 1. **Transport Layer**

- **Tokio TcpListener** - Async TCP socket
- **Axum Server** - HTTP/1.1 and HTTP/2 support

### 2. **Middleware Stack** (Applied bottom-to-top)

```rust
Router::new()
    .merge(routes)
    .layer(main_response_mapper)    // ← Applied 3rd (outermost)
    .layer(mw_ctx_resolver)          // ← Applied 2nd
    .layer(CookieManagerLayer::new()) // ← Applied 1st (innermost)
```

### 3. **Route Layer** (Specific to certain routes)

```rust
routes_apis
    .route_layer(mw_require_auth)  // Only applies to /api/* routes
```

### 4. **Handler Layer**

- Business logic
- Data validation
- Model interaction

### 5. **Model Layer**

- Data access
- Business rules
- State management (Arc<Mutex<...>>)

## 🎭 Component Responsibilities

### **ModelController**

```rust
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}
```

- **Responsibility:** Data management and business logic
- **Thread-safety:** Arc<Mutex\<T>> for concurrent access
- **Pattern:** Repository pattern

### **Ctx (Context)**

```rust
pub struct Ctx {
    user_id: u64,
}
```

- **Responsibility:** User session information
- **Lifetime:** Request-scoped
- **Storage:** Request extensions

### **Error Handling**

```rust
pub enum Error {
    LoginFail,
    TicketDeleteFailIdNotFound { id: u64 },
    // ...
}
```

- **Responsibility:** Application-wide error types
- **Pattern:** Centralized error handling
- **Client safety:** Internal errors never exposed

## 🆚 Comparison with Express.js

| Axum                            | Express.js                  | Notes                   |
| ------------------------------- | --------------------------- | ----------------------- |
| `Router::new()`                 | `express()`                 | Application entry point |
| `.layer(middleware)`            | `app.use(middleware)`       | Middleware registration |
| `.route_layer(auth)`            | `router.use(auth)`          | Scoped middleware       |
| `.route("/path", get(handler))` | `app.get("/path", handler)` | Route registration      |
| `State<T>`                      | `req.app.locals`            | Shared state            |
| `ctx: Ctx`                      | `req.user`                  | User context            |
| `Json<T>`                       | `req.body` / `res.json()`   | JSON handling           |
| `Result<T, Error>`              | `try/catch`                 | Error handling          |

## 🔐 Authentication Flow

```bash
┌──────────────┐
│   /api/login │
└──────┬───────┘
       │
       ↓
┌─────────────────────────────────┐
│ 1. Validate credentials         │
│ 2. Generate token: user-{id}.{} │
│ 3. Set cookie: auth-token       │
└──────┬──────────────────────────┘
       │
       ↓
┌────────────────────────────────┐
│   Client stores cookie         │
└──────┬─────────────────────────┘
       │
       ↓ (Subsequent requests)
┌────────────────────────────────┐
│ Cookie sent with every request │
└──────┬─────────────────────────┘
       │
       ↓
┌────────────────────────────────┐
│ mw_ctx_resolver extracts token │
│ → Parses user_id from token    │
│ → Creates Ctx(user_id)         │
│ → Stores in request.extensions │
└──────┬─────────────────────────┘
       │
       ↓
┌────────────────────────────────┐
│ mw_require_auth validates Ctx  │
│ → Extracts Ctx from extensions │
│ → Returns 403 if not present   │
└──────┬─────────────────────────┘
       │
       ↓
┌────────────────────────────────┐
│ Handler receives Ctx           │
│ → Knows user_id of requester   │
│ → Can enforce authorization    │
└────────────────────────────────┘
```

## 🎯 Key Design Patterns

### 1. **Extractor Pattern**

Type-safe parameter injection:

```rust
async fn handler(
    State(mc): State<ModelController>,  // Shared state
    ctx: Ctx,                            // User context
    Json(body): Json<CreateTicket>,      // Request body
) -> Result<Json<Ticket>> {
    // All parameters automatically extracted!
}
```

### 2. **Result Pattern**

Explicit error handling:

```rust
async fn operation() -> Result<T> {
    let value = fallible_operation()?;  // Auto-propagate errors
    Ok(value)
}
```

### 3. **Middleware Chain Pattern**

Composable request/response processing:

```rust
request
  → layer1
  → layer2
  → handler
  → layer2
  → layer1
  → response
```

### 4. **Extension Storage Pattern**

Type-safe metadata passing:

```rust
// Store
req.extensions_mut().insert(ctx);

// Retrieve
let ctx = req.extensions().get::<Ctx>()?;
```

## 🚀 Performance Characteristics

- **Async/await:** Non-blocking I/O with Tokio
- **Zero-copy:** References and borrowing minimize allocations
- **Type erasure:** `impl Trait` for efficient polymorphism
- **Compile-time checks:** Most errors caught before runtime
- **Arc<Mutex\<T>>:** Minimal overhead for shared state

## 📊 Scalability Considerations

### Current Architecture

- **In-memory storage:** `Vec<Option<Ticket>>`
- **Single-process:** All state in memory
- **Lock contention:** Mutex on every ticket operation

### Production Improvements

```rust
// Replace with database
pub struct ModelController {
    db_pool: Arc<sqlx::PgPool>,  // Connection pool
}

// Remove mutex, use DB transactions
pub async fn create_ticket(&self, ctx: Ctx, ticket: TicketForCreate)
    -> Result<Ticket>
{
    sqlx::query_as!(...)
        .fetch_one(&self.db_pool)
        .await
}
```

## 🔍 Debugging Tips

1. **Enable detailed logs:**

   ```rust
   println!("->> {:<12} - {}", "COMPONENT", message);
   ```

2. **Use `#[axum::debug_handler]`:**

   ```rust
   #[axum::debug_handler]
   async fn handler(...) -> Result<...> {
       // Better compile errors
   }
   ```

3. **Inspect request flow:**
   - Add println!() in each middleware
   - Check layer ordering
   - Verify extractor order

## ✅ Key Takeaways

1. **Layers apply bottom-to-top** - Last layer is outermost
2. **route_layer is scoped** - Only affects specific routes
3. **Extractors are type-safe** - Compiler validates parameters
4. **Errors are values** - Explicit in function signatures
5. **Extensions pass data** - Type-safe request/response metadata

## 📚 Next Steps

- [Project Structure](./02-project-structure.md) - Dive into the code organization
- [Middleware & Layers](./06-middleware-layers.md) - Deep dive into layer system
- [Extractors](./07-extractors.md) - Understanding parameter injection
