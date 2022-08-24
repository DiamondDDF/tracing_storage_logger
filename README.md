# Tracing Storage Logger Crate
### Integrates Tracing and File-Rotate Crates
### Very preliminary.
Creates a thread local tracing subscriber
 that keeps rotating logs
 via standard tracing crate functions and macros.
 This particular example has project_root/logs/ as the root path.
 The name of the log file will either be the name of 
 the span, or the name of "path" if specified.
 Pretty simple, not perfect. Anyone is welcome to improve on this.
```rust
use tracing_storage_logger::prelude::*;

fn main() {
  
    let logger = Logger::new(
        PathBuf::from(r"logs" ContentLimit::Bytes(1024));
        function_a();
}

#[instrument(level = "trace")]
fn function_a(){
    info!(message = "Inside function a", path = "Transactions", level = "error");
    warn!("Exiting function a");
}
#[instrument(level = "debug")]
fn function_b(){
    info!("Inside function b");
    function_c();
    warn!("Exiting function b");

}

#[instrument]
fn function_c(){
    info!("Inside function c");
    error!("Exiting function c");
}
```
