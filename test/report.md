This is our Rust project so far:

main.rs:

```rust
$(cat src/main.rs)
```

tests/cli.rs

```rust
$(cat tests/cli.rs)
```

Duplicate the current from_file test to add a new integration test.

This variation uses the template:

```
Hello, $(echo hello world)!

>(cat)
```

Pipe the string "How are you?" to txt-sh

Expect the output:

```
Hello, world!

How are you?
```

Respond just with the new test
