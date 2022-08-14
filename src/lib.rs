pub mod logger;
pub mod custom_layer;
pub mod prelude{
    pub use super::*;
    pub use std::{path::PathBuf};
    pub use tracing::{debug_span, instrument, info, warn, error, info_span, Subscriber};
    pub use file_rotate::{ContentLimit};
    pub use super::logger::Logger;

}