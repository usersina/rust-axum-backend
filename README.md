# Rust Axum Backend

## Todo

- [ ] Fully grasp the Rust concepts used in this project. [See video](https://www.youtube.com/watch?v=XZtlD_m59sM&list=PL7r-PXl6ZPcCTTxjmsb9bFZB9i01fAtI7&index=8&pp=iAQB)
- [ ] Make the project production-ready. [See video](https://youtu.be/3cA_mk4vdWY?list=PL7r-PXl6ZPcCTTxjmsb9bFZB9i01fAtI7)
- [ ] Refine more with Sea-Query + SQLX + ModQL. [See video](https://youtu.be/-dMH9UiwKqg?list=PL7r-PXl6ZPcCTTxjmsb9bFZB9i01fAtI7)
- [ ] Transform to use workspaces. (See [1st video](https://youtu.be/zUxF0kvydJs?list=PL7r-PXl6ZPcCTTxjmsb9bFZB9i01fAtI7) and [2nd video](https://youtu.be/iCGIqEWWTcA?list=PL7r-PXl6ZPcCTTxjmsb9bFZB9i01fAtI7))

## Getting Started

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

## Resources

- <https://youtu.be/XZtlD_m59sM>
- <https://youtu.be/3cA_mk4vdWY?list=PL7r-PXl6ZPcCTTxjmsb9bFZB9i01fAtI7>
