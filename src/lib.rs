//! This crate is for easily creating rotating logs using the tracing crate.
//! I plan on using it for every project, so it will improve. 
//! Already it's better than solutions most other languages might have, I think.
//! It works. But it's still very preliminary.
//! # Examples
//! ```
//! // prelude includes everything you need, including time zones:
//! use tracing_storage_logger::prelude::*;
//! 
//! fn main() {
//!     // It's going to complain this is an unused variable. That's fine. Use an "_" underscore if you wish:
//!     let logger = Logger::new(
//!         // Root path for all logs would be [project_root]/logs,in this case:
//!         PathBuf::from(r"logs"),
//!         // Size of each file before rotating and adding a date:
//!         ContentLimit::Bytes(1024),
//!         // max number of files to archive for each file name:
//!         4,
//!         chrono_tz::US::Eastern
//!     );
//!     info!(message = "🍺🍺🍺 Cheers!", path = "general");
//!     info!(message = "🌈🌈🌈 Peace and beauty", path = "general");
//!     function_a();
//! }
//! 
//! #[instrument(level = "trace")]
//! fn function_a(){
//!     info!(message = "Inside function a", path = "Transactions", level = "error");
//!     function_b();
//! 
//!     warn!("Exiting function a");
//! }
//! 
//! #[instrument(level = "debug")]
//! fn function_b(){
//!     info!("Inside function b");
//!     function_c("Some message".into());
//!     warn!("Exiting function b");
//! 
//! }
//! 
//! #[instrument]
//! fn function_c(some_argument: &str){
//!     info!("Inside function c");
//!     error!("Exiting function c");
//! }
//! ```

pub mod logger;
pub mod custom_layer;
pub mod prelude{
    pub use std::{path::PathBuf};
    pub use tracing::{debug_span, instrument, info, warn, error, trace, info_span, error_span, warn_span, Subscriber};
    pub use file_rotate::{ContentLimit};
    pub use crate::logger::Logger;
    pub use chrono_tz::Tz;
    pub use chrono_tz;
}