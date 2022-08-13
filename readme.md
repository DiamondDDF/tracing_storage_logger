# Tracing Crate

```rust
// the span will have the fields `user.name = "ferris"` and
// `user.email = "ferris@rust-lang.org"`.
span!(Level::TRACE, "login", user.name, user.email);
```
```json
{
  "fields": {
    "a_bool": true,
    "answer": 42,
    "message": "first example",
    "test.name": "Testing123"
  },
  "level": "Level(Info)",
  "name": "event src/main.rs:18",
  "target": "tracing_storage_logger"
}
```