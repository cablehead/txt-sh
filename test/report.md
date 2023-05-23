This is our Rust project so far:

main.rs:

```rust
$(cat src/main.rs)
```

tests/cli.rs

```rust
$(cat tests/cli.rs)
```

We've just added the new failing test: `from_file_with_pipe`

Update main.rs so to allow the pattern <(...) when a FILE is supplied. Pipe txt-sh's stdin the spawn child process
