# Rust Axum Backend

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
