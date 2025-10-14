# Error Handling in Rust

A comprehensive guide to Rust's error handling system, with comparisons to TypeScript.

## 🎯 Core Philosophy

**Rust:** Errors are **values**, explicitly part of function signatures.

**TypeScript:** Errors are **exceptions**, hidden from type signatures.

```typescript
// TypeScript - errors are invisible
async function deleteTicket(id: number): Promise<Ticket> {
  // Might throw, might not - caller doesn't know!
  if (!exists(id)) throw new Error('Not found')
  return ticket
}
```

```rust
// Rust - errors are explicit
async fn delete_ticket(id: u64) -> Result<Ticket> {
    //                              ^^^^^^
    //                              Tells caller: "This can fail!"
    if !exists(id) {
        return Err(Error::TicketDeleteFailIdNotFound { id });
    }
    Ok(ticket)
}
```

## 📦 The Result Type

### Definition

```rust
pub enum Result<T, E> {
    Ok(T),   // Success - contains value
    Err(E),  // Failure - contains error
}
```

### Your Application's Result

```rust
// In error.rs
pub type Result<T> = core::result::Result<T, Error>;
//       ^^^^^^                                ^^^^^
//       Shorthand                             Your error type

// Now you can write:
fn operation() -> Result<Ticket> { ... }

// Instead of:
fn operation() -> core::result::Result<Ticket, Error> { ... }
```

## 🔄 The `?` Operator - Error Propagation

### Basic Usage

```rust
// Without ?
fn operation() -> Result<Data> {
    let value = match fallible_call() {
        Ok(v) => v,
        Err(e) => return Err(e),  // Propagate error
    };
    Ok(value)
}

// With ? (equivalent)
fn operation() -> Result<Data> {
    let value = fallible_call()?;  // Auto-propagates errors!
    Ok(value)
}
```

### TypeScript Comparison

```typescript
// TypeScript - manual propagation
async function operation(): Promise<Data> {
    try {
        const value = await fallibleCall();
        return value;
    } catch (e) {
        throw e;  // Re-throw
    }
}

// Rust equivalent with ?
async fn operation() -> Result<Data> {
    let value = fallible_call().await?;
    Ok(value)
}
```

### Chaining Multiple Operations

```rust
async fn complex_operation() -> Result<Output> {
    let step1 = fetch_data().await?;      // Fails → returns Err
    let step2 = process(step1)?;          // Fails → returns Err
    let step3 = validate(step2)?;         // Fails → returns Err
    Ok(finalize(step3))                   // Success → returns Ok
}
```

## 🎭 Your Error Type

### Enum Definition

```rust
#[derive(Clone, Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    // Simple variants
    LoginFail,
    AuthFailNoAuthTokenCookie,

    // Variants with data
    TicketDeleteFailIdNotFound { id: u64 },

    // Variants to skip in serialization
    #[serde(skip)]
    DatabaseError { cause: String },
}
```

### Why an Enum?

**Type-safe error handling** - compiler forces you to handle all cases:

```rust
match error {
    Error::LoginFail => { /* handle login failure */ },
    Error::TicketDeleteFailIdNotFound { id } => {
        println!("Ticket {} not found", id);
    },
    // Compiler error if you forget a case!
}
```

**TypeScript Equivalent:**

```typescript
// Discriminated union
type AppError =
  | { type: 'LoginFail' }
  | { type: 'TicketNotFound'; id: number }
  | { type: 'DatabaseError'; cause: string }

// But TypeScript won't enforce exhaustive checks
function handleError(err: AppError) {
  switch (err.type) {
    case 'LoginFail': // ...
    case 'TicketNotFound': // ...
    // Forgot DatabaseError? TypeScript won't complain!
  }
}
```

## 🌐 Converting Errors to HTTP Responses

### The IntoResponse Trait

```rust
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        // 1. Create generic 500 response
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // 2. Store error in extensions (for middleware to process)
        response.extensions_mut().insert(self);

        response
    }
}
```

### Client-Safe Error Conversion

```rust
impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            // Business logic errors - safe to expose
            Self::LoginFail => {
                (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL)
            },
            Self::TicketDeleteFailIdNotFound { .. } => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            },

            // Infrastructure errors - hide details
            Self::DatabaseError { .. } => {
                (StatusCode::INTERNAL_SERVER_ERROR, ClientError::SERVICE_ERROR)
            },
        }
    }
}
```

### Client Error Types

```rust
#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}
```

**Why separate?** Never expose internal error details to clients!

```rust
// Internal error (detailed)
Error::DatabaseError {
    cause: "Connection pool exhausted: timeout after 30s"
}

// Client sees (generic)
{
    "error": {
        "type": "SERVICE_ERROR",
        "req_uuid": "abc-123..."
    }
}
```

## 🔍 Pattern Matching on Errors

### Exhaustive Matching

```rust
fn handle_error(error: Error) -> String {
    match error {
        Error::LoginFail => "Invalid credentials".to_string(),

        Error::TicketDeleteFailIdNotFound { id } => {
            format!("Ticket {} not found", id)
        },

        Error::AuthFailNoAuthTokenCookie => {
            "No auth token".to_string()
        },

        // Must handle ALL variants or use catch-all
        _ => "Unknown error".to_string(),
    }
}
```

### Grouping Similar Errors

```rust
match error {
    // Group authentication errors
    Error::AuthFailNoAuthTokenCookie
    | Error::AuthFailTokenWrongFormat
    | Error::AuthFailCtxNotInRequestExt => {
        log_auth_failure();
        (StatusCode::FORBIDDEN, ClientError::NO_AUTH)
    },

    // Handle others individually
    Error::LoginFail => { /* ... */ },
}
```

## 🛠️ Error Creation Patterns

### Simple Errors

```rust
// Return an error
return Err(Error::LoginFail);

// With ?
some_condition
    .then(|| value)
    .ok_or(Error::LoginFail)?;
```

### Errors with Data

```rust
// Capture context
return Err(Error::TicketDeleteFailIdNotFound { id });

// From Option
ticket_option.ok_or(Error::TicketDeleteFailIdNotFound { id })?;
```

### Converting from Other Error Types

```rust
// Manual conversion
match std::fs::read_to_string("config.json") {
    Ok(content) => content,
    Err(io_err) => return Err(Error::ConfigError(io_err.to_string())),
}

// Better: Implement From trait (see advanced section)
```

## 🚀 Advanced: The `From` Trait

### Automatic Error Conversion

```rust
// Add to your Error enum
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::DatabaseError {
            cause: format!("IO Error: {}", err)
        }
    }
}

// Now ? automatically converts!
async fn load_config() -> Result<Config> {
    let content = tokio::fs::read_to_string("config.json").await?;
    //                                                           ^
    //                                                           Automatically converts io::Error to Error!
    Ok(serde_json::from_str(&content)?)
}
```

### Multiple Error Type Conversions

```rust
impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::SerializationError(err.to_string())
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Error::DatabaseError {
            operation: "query".to_string(),
            cause: err.to_string(),
        }
    }
}

// Now all these convert automatically with ?
async fn complex_operation() -> Result<Data> {
    let json = read_file("data.json").await?;  // io::Error → Error
    let data: Data = serde_json::from_str(&json)?;  // serde_json::Error → Error
    save_to_db(&data).await?;  // sqlx::Error → Error
    Ok(data)
}
```

## 📊 Error Handling Strategies

### Strategy 1: Propagate Everything

```rust
// Simple: Let caller handle errors
async fn operation() -> Result<T> {
    let a = step1()?;
    let b = step2()?;
    let c = step3()?;
    Ok(finalize(a, b, c))
}
```

**Use when:** Function is a simple orchestrator

### Strategy 2: Transform Errors

```rust
// Add context to errors
async fn operation() -> Result<T> {
    let value = step1()
        .await
        .map_err(|_| Error::OperationFailed)?;
    Ok(value)
}
```

**Use when:** You want to hide implementation details

### Strategy 3: Handle and Continue

```rust
// Handle non-critical errors
async fn operation() -> Result<T> {
    let metrics = match collect_metrics().await {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Metrics failed: {}", e);
            Metrics::default()  // Use fallback
        }
    };

    // Continue with main logic
    Ok(process(metrics))
}
```

**Use when:** Errors are non-critical

### Strategy 4: Recover from Errors

```rust
// Retry on failure
async fn operation() -> Result<T> {
    for attempt in 1..=3 {
        match try_operation().await {
            Ok(result) => return Ok(result),
            Err(e) if attempt < 3 => {
                eprintln!("Attempt {} failed: {}", attempt, e);
                tokio::time::sleep(Duration::from_secs(1)).await;
            },
            Err(e) => return Err(e),
        }
    }
    unreachable!()
}
```

**Use when:** Transient failures expected

## 🎓 Best Practices

### ✅ DO

```rust
// Return Result from fallible operations
async fn create_user(name: String) -> Result<User> { ... }

// Use ? for error propagation
let user = find_user(id)?;

// Match on specific error variants
match error {
    Error::NotFound => { /* specific handling */ },
    _ => { /* generic handling */ }
}

// Add context to errors
.map_err(|e| Error::ConfigError(format!("Failed to load: {}", e)))?

// Use Option::ok_or to convert None to Error
user.ok_or(Error::UserNotFound { id })?
```

### ❌ DON'T

```rust
// Don't unwrap in production code
let user = find_user(id).unwrap();  // PANIC! 💥

// Don't ignore errors silently
find_user(id);  // Compiler warning: unused Result

// Don't use generic error messages
return Err(Error::GenericError);  // Not helpful!

// Don't panic in library code
if invalid { panic!("Invalid state"); }  // Use Result instead
```

### 🟡 SOMETIMES OK

```rust
// expect() with good message (initialization code)
let config = load_config()
    .expect("Config file required to start");

// unwrap() when you've proven it can't fail
let number: u32 = "42".parse().unwrap();  // Literal always parses

// unwrap() in tests
#[test]
fn test_something() {
    let result = operation().unwrap();  // Tests can panic
}
```

## 🔧 Debugging Errors

### Print Debug Information

```rust
// Debug print
println!("Error: {:?}", error);

// Pretty debug print
println!("Error: {:#?}", error);

// Display print (user-friendly)
println!("Error: {}", error);
```

### Using `dbg!` Macro

```rust
// Returns value after printing
let config = dbg!(load_config()?);
// Prints: [src/main.rs:42] load_config()? = Config { ... }
```

### Error Context with anyhow (Tests)

```rust
use anyhow::{Context, Result};

#[tokio::test]
async fn test_flow() -> Result<()> {
    let user = create_user("Alice")
        .await
        .context("Failed to create user")?;

    let ticket = create_ticket(user.id)
        .await
        .context("Failed to create ticket")?;

    Ok(())
}

// Error output:
// Failed to create ticket
//
// Caused by:
//     0: Failed to create user
//     1: Database connection timeout
```

## 📚 Common Patterns

### Pattern: Option to Result

```rust
// Convert Option to Result
let value = option.ok_or(Error::NotFound)?;

// With custom error per case
let value = option.ok_or_else(|| Error::NotFound { id })?;
```

### Pattern: Early Return

```rust
fn validate(data: &Data) -> Result<()> {
    if data.name.is_empty() {
        return Err(Error::ValidationFailed {
            field: "name".to_string()
        });
    }

    if data.age < 0 {
        return Err(Error::ValidationFailed {
            field: "age".to_string()
        });
    }

    Ok(())
}
```

### Pattern: Combining Results

```rust
// All must succeed
async fn all_succeed() -> Result<()> {
    let a = operation_a()?;
    let b = operation_b()?;
    let c = operation_c()?;
    Ok(())
}

// Collect results from iterator
let results: Result<Vec<T>> = items
    .iter()
    .map(|item| process(item))
    .collect();  // Fails on first error
```

## ✅ Key Takeaways

1. **Errors are values** - Part of the type signature
2. **`?` operator** - Concise error propagation
3. **Pattern matching** - Compiler-enforced exhaustiveness
4. **Result\<T>** - Your application's Result type
5. **IntoResponse** - Required for returning errors from handlers
6. **Separate internal/client errors** - Never expose internals
7. **`From` trait** - Automatic error conversion

## 📚 Next Steps

- [Response Mapping](./08-response-mapping.md) - How errors become HTTP responses
- [Traits & Type System](./09-traits-type-system.md) - Understanding IntoResponse
- [Error Handling Strategies](./13-error-strategies.md) - Advanced techniques
