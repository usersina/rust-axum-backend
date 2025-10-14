# Rust Axum Backend

A learning-focused backend application built with Rust and Axum, demonstrating best practices for web API development.

## ✨ Features

- 🚀 **Async/await** with Tokio runtime
- 🔐 **Cookie-based authentication** with middleware
- 🎯 **Type-safe error handling** with custom Error enum
- 📝 **Request logging** with UUID tracking
- 🧅 **Layered middleware architecture**
- ✅ **Strongly typed** extractors and responses
- 🧪 **Integration tests** with httpc-test

## 📚 Documentation

Comprehensive documentation for learning Rust and Axum:

- **[Documentation Home](./docs/README.md)** - Start here!
- [Architecture Overview](./docs/01-architecture-overview.md) - System design and request flow
- [Project Structure](./docs/02-project-structure.md) - Codebase organization
- [Rust Fundamentals](./docs/03-rust-fundamentals.md) - Core Rust concepts with TypeScript comparisons
- [Error Handling](./docs/05-error-handling.md) - Result types and error propagation
- [Middleware & Layers](./docs/06-middleware-layers.md) - Understanding the layer system

Perfect for developers coming from **TypeScript/Node.js**!

## 🚀 Getting Started

- Install the dependencies

```bash
cargo build
```

- Install cargo-watch globally

```bash
cargo install cargo-watch
```

- Run the server in watch mode

```bash
cargo watch -q -c -w src/ -x run

# Also for quick dev, run tests in watch mode
cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
```

## 🏗️ Project Structure

```bash
src/
├── main.rs              # Application entry point & router setup
├── error.rs             # Error types and HTTP conversion
├── ctx.rs               # User context (session)
├── model.rs             # Data models and business logic
├── log.rs               # Request logging
└── web/                 # Web layer
    ├── mod.rs           # Module exports
    ├── mw_auth.rs       # Authentication middleware
    ├── routes_login.rs  # Login endpoints
    └── routes_ticket.rs # Ticket CRUD API
```

## 🧪 Running Tests

```bash
# Run all tests
cargo test

# Run tests in watch mode
cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"

# Run specific test
cargo test quick_dev
```

## 📖 API Endpoints

### Authentication

- `GET /api/login?username=demo&pwd=welcome` - Login and set auth cookie

### Tickets (Protected)

- `GET /api/tickets` - List all tickets
- `POST /api/tickets` - Create a ticket

  ```json
  { "title": "Fix bug" }
  ```

- `DELETE /api/tickets/:id` - Delete a ticket

## 🎓 Learning Path

1. **Start with the docs** - [Documentation Home](./docs/README.md)
2. **Understand the architecture** - See [Architecture Overview](./docs/01-architecture-overview.md)
3. **Read the code** - Follow the request flow in `main.rs`
4. **Run the tests** - Experiment with the API
5. **Modify and extend** - Add your own routes and features

## 🔧 Technologies

- **[Axum](https://github.com/tokio-rs/axum)** - Web framework
- **[Tokio](https://tokio.rs/)** - Async runtime
- **[Serde](https://serde.rs/)** - Serialization
- **[Tower](https://github.com/tower-rs/tower)** - Middleware
- **[tower-cookies](https://github.com/imbolc/tower-cookies)** - Cookie management

## 📝 Todo

- [ ] Fully grasp the Rust concepts used in this project. [See video](https://www.youtube.com/watch?v=XZtlD_m59sM&list=PL7r-PXl6ZPcCTTxjmsb9bFZB9i01fAtI7&index=8&pp=iAQB)
- [ ] Make the project production-ready. [See video](https://youtu.be/3cA_mk4vdWY?list=PL7r-PXl6ZPcCTTxjmsb9bFZB9i01fAtI7)
- [ ] Refine more with Sea-Query + SQLX + ModQL. [See video](https://youtu.be/-dMH9UiwKqg?list=PL7r-PXl6ZPcCTTxjmsb9bFZB9i01fAtI7)
- [ ] Transform to use workspaces. (See [1st video](https://youtu.be/zUxF0kvydJs?list=PL7r-PXl6ZPcCTTxjmsb9bFZB9i01fAtI7) and [2nd video](https://youtu.be/iCGIqEWWTcA?list=PL7r-PXl6ZPcCTTxjmsb9bFZB9i01fAtI7))

## 🤝 Contributing

This is a learning project. Feel free to:

- Open issues for questions or clarifications
- Submit PRs to improve documentation
- Share your own learning experiences

## 📚 External Resources

- [The Rust Book](https://doc.rust-lang.org/book/) - Official Rust guide
- [Axum Documentation](https://docs.rs/axum/latest/axum/) - Framework docs
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial) - Async runtime
- [Original Tutorial Series](https://www.youtube.com/watch?v=XZtlD_m59sM&list=PL7r-PXl6ZPcCTTxjmsb9bFZB9i01fAtI7) - YouTube playlist

## 📄 License

This project is for educational purposes.
