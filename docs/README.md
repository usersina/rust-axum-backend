# Rust Axum Backend - Documentation

Welcome to the documentation for this Rust Axum backend project! This guide is designed for developers who are new to Rust but have experience with other backend frameworks (especially TypeScript/Node.js).

## 📚 Table of Contents

### Getting Started

- [Architecture Overview](./01-architecture-overview.md) - High-level system design and request flow
- [Project Structure](./02-project-structure.md) - Understanding the codebase layout

### Core Concepts

- [Rust Fundamentals](./03-rust-fundamentals.md) - Essential Rust concepts with TypeScript analogies
- [Ownership & Borrowing](./04-ownership-borrowing.md) - Understanding Rust's memory model
- [Error Handling](./05-error-handling.md) - Result types, error propagation, and the `?` operator

### Axum Framework

- [Middleware & Layers](./06-middleware-layers.md) - Request/response interception and the layer system
- [Extractors](./07-extractors.md) - Type-safe dependency injection and parameter extraction
- [Response Mapping](./08-response-mapping.md) - Converting handlers to HTTP responses

### Advanced Topics

- [Traits & Type System](./09-traits-type-system.md) - Traits, derive macros, and IntoResponse
- [Async & Concurrency](./10-async-concurrency.md) - Arc, Mutex, Send, Sync, and async/await
- [Module System](./11-module-system.md) - Code organization and visibility

### Patterns & Best Practices

- [Common Patterns](./12-common-patterns.md) - Reusable patterns found in this codebase
- [Error Handling Strategies](./13-error-strategies.md) - Advanced error handling techniques
- [Testing Guide](./14-testing-guide.md) - How to test Axum applications

## 🎯 For TypeScript Developers

If you're coming from TypeScript/Node.js, start here:

1. [Rust Fundamentals](./03-rust-fundamentals.md) - See side-by-side comparisons
2. [Architecture Overview](./01-architecture-overview.md) - Compare with Express.js
3. [Error Handling](./05-error-handling.md) - Different from try/catch!

## 🚀 Quick Reference

- **Running the project:** `cargo run`
- **Running tests:** `cargo test`
- **Checking code:** `cargo clippy`
- **Formatting:** `cargo fmt`

## 📖 Learning Path

### Beginner (Start Here)

1. Architecture Overview
2. Rust Fundamentals
3. Error Handling
4. Middleware & Layers

### Intermediate

1. Extractors
2. Response Mapping
3. Traits & Type System
4. Common Patterns

### Advanced

1. Async & Concurrency
2. Module System
3. Error Handling Strategies
4. Testing Guide

## 🆘 Troubleshooting

Common issues and solutions:

- **"trait bound not satisfied"** → See [Traits & Type System](./09-traits-type-system.md#intoresponse-trait)
- **"cannot borrow as mutable"** → See [Ownership & Borrowing](./04-ownership-borrowing.md)
- **Middleware order confusion** → See [Middleware & Layers](./06-middleware-layers.md#layer-ordering)
- **Extractor errors** → See [Extractors](./07-extractors.md#common-mistakes)

## 📚 External Resources

- [The Rust Book](https://doc.rust-lang.org/book/) - Official Rust guide
- [Axum Documentation](https://docs.rs/axum/latest/axum/) - Framework docs
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn by doing
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial) - Async runtime guide

## 🤝 Contributing

Found an error or want to improve the docs? Please open an issue or PR!

## 📝 License

Documentation is provided as-is for learning purposes.
