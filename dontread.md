# Docs
### Integrates Tracing and File-Rotate Crates
### Very preliminary.
Creates a thread local tracing subscriber
 that keeps rotating logs
 via standard tracing crate functions and macros.
 This particular example has project_root/logs/ as the root path.
 The name of the log file will either be the name of 
 the span, or the name of "path" if specified.
 Pretty simple, not perfect. Anyone is welcome to improve on this.

## Example
```rust
use tracing_storage_logger::prelude::*;

fn main() {
    // It's going to complain this is an unused variable. That's fine. Use an "_" underscore if you wish:
    let logger = Logger::new(
        PathBuf::from(r"logs"), 
        ContentLimit::Bytes(1024),
        4,
        chrono_tz::US::Eastern
    );
    info!(message = "🍺🍺🍺 Cheers!", path = "general");
    info!(message = "🌈🌈🌈 Peace and beauty", path = "general");//
    function_a();
}

#[instrument(level = "trace")]
fn function_a(){
    info!(message = "Inside function a", path = "Transactions", level = "error");
    function_b();

    warn!("Exiting function a");
}

#[instrument(level = "debug")]
fn function_b(){
    info!("Inside function b");
    function_c("Some message".into());
    warn!("Exiting function b");

}

#[instrument]
fn function_c(some_argument: &str){
    info!("Inside function c");
    error!("Exiting function c");
}
```
