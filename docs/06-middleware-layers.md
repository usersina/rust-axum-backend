# Middleware & Layers

Understanding Axum's middleware system and layer ordering.

## 🧅 The Onion Model

Middleware in Axum works like wrapping an onion - each layer wraps around the previous one.

```plaintext
┌─────────────────────────────────────────────┐
│ Layer 3 (Outermost)                         │
│  ┌───────────────────────────────────────┐  │
│  │ Layer 2 (Middle)                      │  │
│  │  ┌─────────────────────────────────┐  │  │
│  │  │ Layer 1 (Innermost)             │  │  │
│  │  │  ┌───────────────────────────┐  │  │  │
│  │  │  │ Your Handler              │  │  │  │
│  │  │  └───────────────────────────┘  │  │  │
│  │  └─────────────────────────────────┘  │  │
│  └───────────────────────────────────────┘  │
└─────────────────────────────────────────────┘

Request flows: 3 → 2 → 1 → Handler → 1 → 2 → 3
```

## 🔄 Layer Execution Order

### Code Order vs Execution Order

```rust
Router::new()
    .route("/api/users", get(handler))
    .layer(layer_c)  // ← Applied 3rd, runs 1st on request
    .layer(layer_b)  // ← Applied 2nd, runs 2nd on request
    .layer(layer_a)  // ← Applied 1st, runs 3rd on request
```

**Request flow:**

```plaintext
Client Request
    ↓
Layer A (outermost)
    ↓
Layer B
    ↓
Layer C (innermost)
    ↓
Handler executes
    ↓
Layer C (innermost)
    ↓
Layer B
    ↓
Layer A (outermost)
    ↓
Client Response
```

### Why Bottom-to-Top?

Think of `.layer()` as **wrapping** the existing router:

```rust
let router = Router::new().route("/api/users", get(handler));
// router = Handler

let router = router.layer(layer_c);
// router = layer_c(Handler)

let router = router.layer(layer_b);
// router = layer_b(layer_c(Handler))

let router = router.layer(layer_a);
// router = layer_a(layer_b(layer_c(Handler)))

// Request hits layer_a first!
```

## 🎯 Your Application's Middleware Stack

### Global Layers (Applied to All Routes)

```rust
let routes_all = Router::new()
    .merge(routes_hello())
    .merge(web::routes_login::routes())
    .nest("/api", routes_apis)
    .layer(middleware::map_response(main_response_mapper))  // ← Layer 3
    .layer(middleware::from_fn_with_state(                  // ← Layer 2
        mc.clone(),
        web::mw_auth::mw_ctx_resolver,
    ))
    .layer(CookieManagerLayer::new())                       // ← Layer 1
    .fallback_service(get_service(ServeDir::new("./")));
```

**Execution order:**

```plaintext
1. CookieManagerLayer      (Layer 1 - Outermost)
   ↓
2. mw_ctx_resolver         (Layer 2 - Middle)
   ↓
3. Routes execute          (Core)
   ↓
4. main_response_mapper    (Layer 3 - Response only)
   ↓
Response to client
```

### Route-Specific Layers

```rust
let routes_apis = web::routes_ticket::routes(mc.clone())
    .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));
```

**Key difference:**

- `.layer()` - Applies to ALL routes (including ones merged later)
- `.route_layer()` - Applies ONLY to routes in THIS router

## 🔍 Tracing a Request

### Example: `POST /api/tickets`

```plaintext
1. Client sends request
   POST /api/tickets
   Cookie: auth-token=user-123.abc
   Body: {"title": "Fix bug"}

2. CookieManagerLayer (Global Layer 1)
   → Parses Cookie header
   → Makes cookies available to handlers
   → Continues to next layer

3. mw_ctx_resolver (Global Layer 2)
   → Extracts auth-token cookie
   → Parses token: "user-123" → user_id = 123
   → Creates Ctx { user_id: 123 }
   → Stores Ctx in req.extensions()
   → Continues to next layer

4. Route matching
   → Matches /api/tickets route
   → Enters routes_apis router

5. mw_require_auth (Route Layer)
   → Attempts to extract Ctx
   → If missing: return 403 error
   → If present: continue to handler

6. create_ticket Handler
   → Extracts: State(mc), Ctx, Json(body)
   → Calls mc.create_ticket(ctx, ticket_fc)
   → Returns: Ok(Json(ticket))

7. Response flows back through layers
   → mw_require_auth (does nothing on response)
   → mw_ctx_resolver (does nothing on response)
   → main_response_mapper:
      • Extracts any errors
      • Logs the request
      • Transforms error to client-safe format
   → CookieManagerLayer (serializes cookies to Set-Cookie header)

8. Client receives response
   201 Created
   Body: {"id": 1, "title": "Fix bug"}
```

## 🛠️ Types of Middleware

### 1. Request Middleware

Processes the request before the handler:

```rust
async fn request_logger(
    req: Request<Body>,
    next: Next,
) -> Result<Response> {
    println!("Request: {} {}", req.method(), req.uri());
    Ok(next.run(req).await)
}
```

### 2. Response Middleware

Processes the response after the handler:

```rust
async fn response_logger(
    req: Request<Body>,
    next: Next,
) -> Result<Response> {
    let res = next.run(req).await;
    println!("Response: {}", res.status());
    Ok(res)
}
```

### 3. Request/Response Middleware

Processes both:

```rust
async fn timing_middleware(
    req: Request<Body>,
    next: Next,
) -> Result<Response> {
    let start = Instant::now();

    // Process request
    let res = next.run(req).await;

    // Process response
    let elapsed = start.elapsed();
    println!("Request took: {:?}", elapsed);

    Ok(res)
}
```

### 4. Extracting Middleware

Uses extractors to get request data:

```rust
async fn auth_middleware(
    cookies: Cookies,  // ← Extractor
    uri: Uri,          // ← Extractor
    req: Request<Body>,
    next: Next,
) -> Result<Response> {
    // Use cookies and uri
    Ok(next.run(req).await)
}
```

### 5. Stateful Middleware

Accesses shared application state:

```rust
async fn rate_limit_middleware(
    State(limiter): State<RateLimiter>,  // ← Shared state
    req: Request<Body>,
    next: Next,
) -> Result<Response> {
    if limiter.check_limit().await {
        Ok(next.run(req).await)
    } else {
        Err(Error::RateLimitExceeded)
    }
}
```

## 🎭 Common Middleware Patterns

### Pattern 1: Early Return

```rust
async fn auth_required(
    ctx: Result<Ctx>,
    req: Request<Body>,
    next: Next,
) -> Result<Response> {
    // Check auth before continuing
    ctx?;  // Return 403 if no Ctx

    // Continue to handler
    Ok(next.run(req).await)
}
```

### Pattern 2: Request Modification

```rust
async fn add_header(
    mut req: Request<Body>,
    next: Next,
) -> Result<Response> {
    // Modify request
    req.headers_mut().insert(
        "X-Custom-Header",
        "value".parse().unwrap()
    );

    Ok(next.run(req).await)
}
```

### Pattern 3: Response Modification

```rust
async fn add_cors_headers(
    req: Request<Body>,
    next: Next,
) -> Result<Response> {
    let mut res = next.run(req).await;

    // Modify response
    res.headers_mut().insert(
        "Access-Control-Allow-Origin",
        "*".parse().unwrap()
    );

    Ok(res)
}
```

### Pattern 4: Error Recovery

```rust
async fn error_recovery(
    req: Request<Body>,
    next: Next,
) -> Result<Response> {
    match next.run(req).await {
        res if res.status().is_success() => Ok(res),
        res if res.status() == StatusCode::NOT_FOUND => {
            // Custom 404 page
            Ok(Html("<h1>Not Found</h1>").into_response())
        },
        res => Ok(res),  // Pass through other errors
    }
}
```

## 📊 Layer vs Route Layer

### When to Use `.layer()`

```rust
Router::new()
    .merge(public_routes)
    .merge(api_routes)
    .layer(logging)      // ← Logs ALL routes
    .layer(cors)         // ← CORS for ALL routes
    .layer(cookies)      // ← Cookies for ALL routes
```

**Use for:**

- Cross-cutting concerns
- Features needed by all routes
- Infrastructure-level functionality

### When to Use `.route_layer()`

```rust
let admin_routes = Router::new()
    .route("/admin/users", get(list_users))
    .route("/admin/settings", get(settings))
    .route_layer(require_admin);  // ← Only admin routes

Router::new()
    .merge(public_routes)   // ← No admin check
    .merge(admin_routes)    // ← Has admin check
```

**Use for:**

- Route-specific authentication
- Different rate limits per endpoint
- Feature flags for specific routes

## 🔄 Middleware with State

### Passing State to Middleware

```rust
// Method 1: from_fn_with_state
.layer(middleware::from_fn_with_state(
    mc.clone(),  // ← State
    my_middleware,
))

async fn my_middleware(
    State(mc): State<ModelController>,  // ← Extract state
    req: Request<Body>,
    next: Next,
) -> Result<Response> {
    // Use mc here
    Ok(next.run(req).await)
}

// Method 2: Closure
.layer(middleware::from_fn(move |req, next| {
    let mc = mc.clone();  // ← Capture from outer scope
    async move {
        // Use mc here
        next.run(req).await
    }
}))
```

## 🎯 Your Application's Middleware

### 1. CookieManagerLayer

**Purpose:** Parse and serialize cookies

**Type:** Third-party (tower-cookies)

**When it runs:**

- Request: Parses `Cookie` header into `Cookies` extractor
- Response: Serializes cookies into `Set-Cookie` header

**Usage in handlers:**

```rust
async fn handler(cookies: Cookies) -> Response {
    let value = cookies.get("auth-token");
    cookies.add(Cookie::new("session", "abc123"));
    // ...
}
```

### 2. mw_ctx_resolver

**Purpose:** Extract user context from auth token

**Type:** Custom middleware

**What it does:**

```rust
pub async fn mw_ctx_resolver(
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response> {
    // 1. Get auth token from cookie
    let auth_token = cookies.get(AUTH_TOKEN)
        .map(|c| c.value().to_string());

    // 2. Parse token
    let result_ctx = auth_token
        .ok_or(Error::AuthFailNoAuthTokenCookie)
        .and_then(parse_token)
        .map(|(user_id, _signature)| Ctx::new(user_id));

    // 3. Store in request extensions (even if error!)
    if let Ok(ctx) = result_ctx {
        req.extensions_mut().insert(ctx);
    }

    // 4. Continue (don't fail here, let mw_require_auth handle it)
    Ok(next.run(req).await)
}
```

**Key insight:** This middleware **always continues**, even on error. It stores the Ctx if available, but lets `mw_require_auth` decide whether to reject the request.

### 3. mw_require_auth

**Purpose:** Reject requests without authentication

**Type:** Custom route layer

**What it does:**

```rust
pub async fn mw_require_auth(
    ctx: Result<Ctx>,  // ← Tries to extract Ctx
    req: Request<Body>,
    next: Next,
) -> Result<Response> {
    ctx?;  // Return 403 if Ctx not present

    Ok(next.run(req).await)
}
```

**Applied to:** Only `/api/*` routes (via `.route_layer()`)

**Why separate from mw_ctx_resolver?**

- `mw_ctx_resolver` - Always runs, attempts to create Ctx
- `mw_require_auth` - Only on protected routes, enforces Ctx exists

### 4. main_response_mapper

**Purpose:** Transform errors and log requests

**Type:** Response mapper (special middleware)

**What it does:**

```rust
async fn main_response_mapper(
    ctx: Result<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    // 1. Extract error from response extensions
    let service_error = res.extensions().get::<Error>();

    // 2. Convert to client-safe error
    let client_error = service_error.map(|e| e.client_status_and_error());

    // 3. Build new response if error
    let error_response = client_error
        .as_ref()
        .map(|(status, err)| {
            (status, Json(json!({
                "error": { "type": err.as_ref(), ... }
            }))).into_response()
        });

    // 4. Log request
    log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

    // 5. Return error response or original
    error_response.unwrap_or(res)
}
```

**Applied with:** `middleware::map_response()`

**Runs:** After handler, before response sent to client

## 🚨 Common Mistakes

### Mistake 1: Wrong Layer Order

```rust
// ❌ BAD - Cookies parsed AFTER auth check
Router::new()
    .route("/api/users", get(handler))
    .layer(auth_middleware)      // Needs cookies!
    .layer(CookieManagerLayer)   // But cookies parsed here
```

```rust
// ✅ GOOD - Cookies parsed FIRST
Router::new()
    .route("/api/users", get(handler))
    .layer(auth_middleware)      // Can use cookies
    .layer(CookieManagerLayer)   // Parsed first (outermost)
```

### Mistake 2: Applying Layer Before Routes

```rust
// ❌ BAD - Layer doesn't affect routes added after
Router::new()
    .layer(logging)              // Only affects routes added before this
    .merge(routes)               // Not logged!
```

```rust
// ✅ GOOD - Routes first, then layers
Router::new()
    .merge(routes)               // Routes defined
    .layer(logging)              // Now wraps all routes
```

### Mistake 3: Using route_layer for Global Concerns

```rust
// ❌ BAD - Have to add to every router
let router1 = Router::new()
    .route("/users", get(handler))
    .route_layer(logging);       // Only /users

let router2 = Router::new()
    .route("/posts", get(handler))
    .route_layer(logging);       // Only /posts

Router::new()
    .merge(router1)
    .merge(router2)
```

```rust
// ✅ GOOD - Use global layer
let router1 = Router::new()
    .route("/users", get(handler));

let router2 = Router::new()
    .route("/posts", get(handler));

Router::new()
    .merge(router1)
    .merge(router2)
    .layer(logging)              // Logs everything
```

## 🎓 Best Practices

### ✅ DO

- **Apply global layers last** (after all routes merged)
- **Order layers by dependency** (cookies before auth)
- **Use route_layer for specific requirements** (admin-only routes)
- **Keep middleware focused** (single responsibility)
- **Log at appropriate levels** (debug, info, error)

### ❌ DON'T

- **Apply layers before defining routes**
- **Use unwrap() in middleware** (propagate errors)
- **Block in middleware** (use async operations)
- **Modify request after calling next()** (too late!)
- **Forget to call next.run()** (request will hang)

## ✅ Key Takeaways

1. **Layers apply bottom-to-top** - Last `.layer()` is outermost
2. **`.layer()` is global** - Affects all routes including merged
3. **`.route_layer()` is scoped** - Only affects routes in that router
4. **Middleware is composable** - Stack layers to build functionality
5. **Order matters** - Dependencies must be in correct order
6. **Always call `next.run()`** - Or request will never complete

## 📚 Next Steps

- [Extractors](./07-extractors.md) - Understanding parameter injection
- [Response Mapping](./08-response-mapping.md) - Deep dive into error transformation
- [Common Patterns](./12-common-patterns.md) - Reusable middleware patterns
