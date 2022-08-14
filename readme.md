# Tracing Storage Logger Crate
### Integrates Tracing and File-Rotate Crates
### Very preliminary.
```rust
fn main() {
  /// creates a thread local tracing subscriber
  /// that logs to project/logs/Transactions
   let logger = Logger::new(PathBuf::from(r"logs"));
    function_a();
}
#[instrument(level = "trace", target = "collection_loop")]
fn function_a(){
    info!(message = "Inside function a", path = "Transactions", level = "error");
    warn!("Exiting function a");
}
// the span will have the fields `user.name = "ferris"` and
// `user.email = "ferris@rust-lang.org"`.
span!(Level::TRACE, "login", user.name, user.email);
```
