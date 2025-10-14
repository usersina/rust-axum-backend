# Rust Fundamentals for TypeScript Developers

A guide to core Rust concepts with TypeScript comparisons.

## 🎯 Philosophy Differences

| Aspect                | TypeScript               | Rust                          |
| --------------------- | ------------------------ | ----------------------------- |
| **Type checking**     | Compile-time (optional)  | Compile-time (mandatory)      |
| **Memory management** | Garbage collected        | Ownership system              |
| **Null handling**     | `null`/`undefined`       | `Option<T>`                   |
| **Error handling**    | Exceptions (`try/catch`) | `Result<T, E>`                |
| **Mutability**        | Default (let/const)      | Explicit (`let` vs `let mut`) |
| **Concurrency**       | Single-threaded + async  | Multi-threaded safe           |

## 🧱 Basic Types

### Primitive Types

```rust
// Numbers
let x: i32 = 42;           // 32-bit signed integer
let y: u64 = 100;          // 64-bit unsigned integer
let z: f64 = 3.14;         // 64-bit float

// Boolean
let active: bool = true;

// Character (Unicode scalar)
let emoji: char = '😀';    // 4 bytes (not 2 like JS)

// String types (complex!)
let s1: &str = "hello";    // String slice (immutable)
let s2: String = String::from("hello");  // Owned string (mutable)
```

**TypeScript equivalent:**

```typescript
let x: number = 42
let y: number = 100
let z: number = 3.14
let active: boolean = true
let emoji: string = '😀'
let s: string = 'hello'
```

### Collections

```rust
// Vector (like Array)
let mut vec: Vec<i32> = vec![1, 2, 3];
vec.push(4);

// Array (fixed size)
let arr: [i32; 3] = [1, 2, 3];

// HashMap (like Object/Map)
use std::collections::HashMap;
let mut map = HashMap::new();
map.insert("key", "value");

// String
let s = String::from("hello");
```

**TypeScript equivalent:**

```typescript
const vec: number[] = [1, 2, 3]
vec.push(4)

const arr: [number, number, number] = [1, 2, 3]

const map = new Map<string, string>()
map.set('key', 'value')

const s: string = 'hello'
```

## 🔒 Ownership & Borrowing

### The Three Rules

1. **Each value has one owner**
2. **When owner goes out of scope, value is dropped**
3. **You can have EITHER:**
   - One mutable reference (`&mut T`), OR
   - Multiple immutable references (`&T`)

### Move Semantics

```rust
let s1 = String::from("hello");
let s2 = s1;  // s1 is MOVED to s2
// println!("{}", s1);  // ❌ ERROR: s1 no longer valid
println!("{}", s2);     // ✅ OK
```

**TypeScript comparison:**

```typescript
let s1 = 'hello'
let s2 = s1 // Both still valid (primitive copied)
console.log(s1) // ✅ OK
console.log(s2) // ✅ OK

let obj1 = { name: 'Alice' }
let obj2 = obj1 // Reference copied (both point to same object)
console.log(obj1.name) // ✅ OK
console.log(obj2.name) // ✅ OK
```

**Rust forces you to think about ownership!**

### Borrowing

```rust
fn calculate_length(s: &String) -> usize {
    //                 ^
    //                 Borrow, don't take ownership
    s.len()
}

let s1 = String::from("hello");
let len = calculate_length(&s1);  // Borrow s1
//                         ^
println!("Length of '{}' is {}", s1, len);  // s1 still valid!
```

### Mutable Borrowing

```rust
fn append_world(s: &mut String) {
    //             ^^^^
    //             Mutable borrow
    s.push_str(" world");
}

let mut s = String::from("hello");
append_world(&mut s);
//           ^^^^
println!("{}", s);  // "hello world"
```

**Rules:**

```rust
let mut s = String::from("hello");

let r1 = &s;      // ✅ Immutable borrow
let r2 = &s;      // ✅ Another immutable borrow
println!("{} {}", r1, r2);

let r3 = &mut s;  // ❌ ERROR: Can't have mutable while immutable exists
```

## 🏗️ Structs

### Definition & Usage

```rust
// Define struct
struct User {
    username: String,
    email: String,
    age: u32,
}

// Implement methods
impl User {
    // Associated function (like static method)
    fn new(username: String, email: String) -> Self {
        Self {
            username,
            email,
            age: 0,
        }
    }

    // Method (has &self parameter)
    fn display(&self) {
        println!("{} <{}>", self.username, self.email);
    }

    // Mutable method
    fn set_age(&mut self, age: u32) {
        self.age = age;
    }
}

// Usage
let mut user = User::new(
    String::from("alice"),
    String::from("alice@example.com")
);
user.display();
user.set_age(25);
```

**TypeScript equivalent:**

```typescript
class User {
  username: string
  email: string
  age: number

  constructor(username: string, email: string) {
    this.username = username
    this.email = email
    this.age = 0
  }

  display(): void {
    console.log(`${this.username} <${this.email}>`)
  }

  setAge(age: number): void {
    this.age = age
  }
}

const user = new User('alice', 'alice@example.com')
user.display()
user.setAge(25)
```

## 🎭 Enums

### Simple Enums

```rust
enum Status {
    Active,
    Inactive,
    Pending,
}

let status = Status::Active;
```

### Enums with Data

```rust
enum Message {
    Quit,                       // No data
    Move { x: i32, y: i32 },   // Named fields
    Write(String),              // Single value
    ChangeColor(i32, i32, i32), // Tuple
}

let msg = Message::Write(String::from("Hello"));
```

**TypeScript equivalent (discriminated union):**

```typescript
type Message =
  | { type: 'Quit' }
  | { type: 'Move'; x: number; y: number }
  | { type: 'Write'; message: string }
  | { type: 'ChangeColor'; r: number; g: number; b: number }

const msg: Message = {
  type: 'Write',
  message: 'Hello',
}
```

### Pattern Matching

```rust
match msg {
    Message::Quit => {
        println!("Quit");
    },
    Message::Move { x, y } => {
        println!("Move to ({}, {})", x, y);
    },
    Message::Write(text) => {
        println!("Write: {}", text);
    },
    Message::ChangeColor(r, g, b) => {
        println!("Color: ({}, {}, {})", r, g, b);
    },
}
```

**Compiler enforces exhaustiveness!** If you forget a case, it won't compile.

## 🎁 Option Type

### Replacing Null

```rust
// Option<T> is an enum with two variants
enum Option<T> {
    Some(T),    // Has a value
    None,       // No value
}

fn find_user(id: u64) -> Option<User> {
    if id == 1 {
        Some(User::new(...))
    } else {
        None
    }
}

// Using Option
match find_user(1) {
    Some(user) => println!("Found: {}", user.username),
    None => println!("Not found"),
}

// Or use methods
let user = find_user(1)
    .unwrap_or_else(|| User::default());
```

**TypeScript equivalent:**

```typescript
function findUser(id: number): User | null {
    if (id === 1) {
        return new User(...);
    } else {
        return null;
    }
}

// Using null
const user = findUser(1);
if (user !== null) {
    console.log(`Found: ${user.username}`);
} else {
    console.log("Not found");
}
```

**Rust forces you to handle None!** Can't use a value without checking.

### Common Option Methods

```rust
let maybe_value: Option<i32> = Some(42);

// Check if Some/None
if maybe_value.is_some() { /* ... */ }
if maybe_value.is_none() { /* ... */ }

// Get value or default
let value = maybe_value.unwrap_or(0);

// Transform value if Some
let doubled = maybe_value.map(|x| x * 2);  // Some(84)

// Chain operations
let result = maybe_value
    .map(|x| x * 2)
    .filter(|x| x > &50)
    .unwrap_or(0);
```

## ✅ Result Type

### Error Handling Without Exceptions

```rust
enum Result<T, E> {
    Ok(T),      // Success
    Err(E),     // Error
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

// Using Result
match divide(10.0, 2.0) {
    Ok(result) => println!("Result: {}", result),
    Err(err) => println!("Error: {}", err),
}

// Or propagate with ?
fn calculate() -> Result<f64, String> {
    let x = divide(10.0, 2.0)?;  // If Err, return early
    let y = divide(x, 3.0)?;
    Ok(y)
}
```

## 🎨 Traits

### Like Interfaces, But More Powerful

```rust
// Define trait
trait Summary {
    fn summarize(&self) -> String;
}

// Implement for a type
struct Article {
    title: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}", self.title, self.content)
    }
}

// Use trait
fn print_summary(item: &impl Summary) {
    println!("{}", item.summarize());
}
```

**TypeScript equivalent:**

```typescript
interface Summary {
  summarize(): string
}

class Article implements Summary {
  title: string
  content: string

  summarize(): string {
    return `${this.title}: ${this.content}`
  }
}

function printSummary(item: Summary): void {
  console.log(item.summarize())
}
```

### Trait Bounds

```rust
// Only types that implement Summary
fn notify<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}

// Multiple traits
fn complex<T: Summary + Clone>(item: &T) {
    // Can call summarize() and clone()
}
```

## 🔄 Generics

```rust
// Generic function
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Generic struct
struct Point<T> {
    x: T,
    y: T,
}

// Generic enum (we've seen these!)
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

## 🔁 Iterators

```rust
let numbers = vec![1, 2, 3, 4, 5];

// Iterate
for num in &numbers {
    println!("{}", num);
}

// Map
let doubled: Vec<i32> = numbers.iter()
    .map(|x| x * 2)
    .collect();

// Filter
let evens: Vec<i32> = numbers.iter()
    .filter(|x| *x % 2 == 0)
    .copied()
    .collect();

// Reduce (fold)
let sum: i32 = numbers.iter().sum();
```

**Very similar to JavaScript!**

## ⚡ Async/Await

```rust
// Async function
async fn fetch_user(id: u64) -> Result<User> {
    let response = http_client.get(url).await?;
    let user = response.json().await?;
    Ok(user)
}

// Async block
let future = async {
    let user = fetch_user(1).await?;
    Ok(user)
};

// Run async function
tokio::spawn(async {
    match fetch_user(1).await {
        Ok(user) => println!("Got user: {}", user.username),
        Err(e) => eprintln!("Error: {}", e),
    }
});
```

**Nearly identical to TypeScript!**

## 🧵 Concurrency Primitives

### Arc - Atomic Reference Counting

```rust
use std::sync::Arc;

let data = Arc::new(vec![1, 2, 3]);
let data_clone = data.clone();  // Just increments counter

// Can share across threads
thread::spawn(move || {
    println!("{:?}", data_clone);
});
```

**Like:** Automatic reference counting (but thread-safe)

### Mutex - Mutual Exclusion

```rust
use std::sync::Mutex;

let counter = Mutex::new(0);

{
    let mut num = counter.lock().unwrap();
    *num += 1;
}  // Lock released here

println!("Count: {}", *counter.lock().unwrap());
```

**Like:** `await mutex.acquire()` in other languages

### Arc + Mutex (Common Pattern)

```rust
use std::sync::{Arc, Mutex};

let shared_data = Arc::new(Mutex::new(vec![]));

let data_clone = shared_data.clone();
thread::spawn(move || {
    let mut data = data_clone.lock().unwrap();
    data.push(42);
});
```

**This is exactly what ModelController uses!**

## 📝 Derive Macros

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

// Now you can:
let p1 = Point { x: 1, y: 2 };
let p2 = p1.clone();           // Clone
println!("{:?}", p1);          // Debug
assert_eq!(p1, p2);            // PartialEq
```

**Common derives:**

- `Debug` - `{:?}` formatting
- `Clone` - `.clone()` method
- `Copy` - Implicit copying (for small types)
- `PartialEq` - `==` operator
- `Eq` - Full equality
- `PartialOrd` - `<`, `>` operators
- `Ord` - Full ordering
- `Hash` - Hashing for HashMap keys
- `Default` - `Default::default()`
- `Serialize`, `Deserialize` - Serde traits

## 🎯 Common Patterns

### Builder Pattern

```rust
struct User {
    username: String,
    email: String,
    age: Option<u32>,
}

impl User {
    fn builder() -> UserBuilder {
        UserBuilder::default()
    }
}

struct UserBuilder {
    username: String,
    email: String,
    age: Option<u32>,
}

impl UserBuilder {
    fn username(mut self, username: String) -> Self {
        self.username = username;
        self
    }

    fn email(mut self, email: String) -> Self {
        self.email = email;
        self
    }

    fn age(mut self, age: u32) -> Self {
        self.age = Some(age);
        self
    }

    fn build(self) -> User {
        User {
            username: self.username,
            email: self.email,
            age: self.age,
        }
    }
}

// Usage
let user = User::builder()
    .username("alice".to_string())
    .email("alice@example.com".to_string())
    .age(25)
    .build();
```

### Newtype Pattern

```rust
// Wrap a type for type safety
struct UserId(u64);
struct PostId(u64);

fn get_user(id: UserId) -> User { ... }
fn get_post(id: PostId) -> Post { ... }

let user_id = UserId(123);
let post_id = PostId(456);

get_user(user_id);      // ✅ OK
// get_user(post_id);   // ❌ ERROR: Type mismatch!
```

## ✅ Key Differences from TypeScript

1. **Ownership** - Values have one owner; think about moves
2. **Borrowing** - Use `&` and `&mut` explicitly
3. **No null** - Use `Option<T>` instead
4. **No exceptions** - Use `Result<T, E>` instead
5. **Explicit mutability** - `let` vs `let mut`
6. **Match exhaustiveness** - Compiler enforces all cases
7. **Traits vs interfaces** - More powerful, can add to existing types
8. **Macros** - Code generation (derive, println!, vec!)
9. **Lifetimes** - Sometimes need to specify (not covered here)
10. **Type inference** - Strong, but sometimes need annotations

## 🎓 Mental Models

### From JavaScript/TypeScript

```typescript
// TypeScript thinking
const user = getUser() // Might be null
if (user) {
  console.log(user.name)
}
```

```rust
// Rust thinking
let user = get_user();  // Option<User>
match user {
    Some(u) => println!("{}", u.name),
    None => println!("No user"),
}

// Or
let user = get_user().unwrap_or(User::default());
```

### Error Handling

```typescript
// TypeScript
try {
  const result = await operation()
  return result
} catch (e) {
  console.error(e)
  throw e
}
```

```rust
// Rust
async fn operation() -> Result<Data> {
    let result = fallible_operation().await?;
    Ok(result)
}
```

## ✅ Key Takeaways

1. **Ownership** - Core concept, prevents memory bugs at compile-time
2. **Option/Result** - Replace null/exceptions with explicit types
3. **Match** - Powerful pattern matching with exhaustiveness checking
4. **Traits** - More flexible than interfaces
5. **Generics** - Similar to TypeScript, but with trait bounds
6. **Async/await** - Familiar syntax, different runtime (Tokio)
7. **Compiler is strict** - But catches bugs early!

## 📚 Next Steps

- [Ownership & Borrowing](./04-ownership-borrowing.md) - Deep dive
- [Error Handling](./05-error-handling.md) - Result and error patterns
- [Traits & Type System](./09-traits-type-system.md) - Advanced type features
