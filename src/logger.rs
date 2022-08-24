use crate::custom_layer::CustomLayer;
use chrono_tz::Tz;
use file_rotate::ContentLimit;
use std::path::PathBuf;
pub use tracing::{debug_span, error, info, info_span, instrument, warn, Subscriber};
use tracing_subscriber::{fmt::Layer, prelude::*};

pub struct Logger;

/// This is the main struct for the end user.
/// * `path` - root path to put all log files in. In messages "path" will be added to the root path.
/// * `file_size_limit` - in bytes.
/// * `max_files` - max number of files of the same name to keep before deleting.
/// * `time_zone` - time is prefixed above all entries.
impl Logger {
    pub fn new(path: PathBuf, limit: ContentLimit, max_files: usize, time_zone: Tz) -> Self {
        // see if init outside of main scope is going to work:
        tracing_subscriber::registry()
            .with(CustomLayer {
                path,
                file_size_limit: limit,
                max_files,
                time_zone,
            })
            .init();

        Logger
    }
}
