use std::{path::PathBuf};
pub use tracing::{debug_span, instrument, info, warn, error, info_span, Subscriber};
use tracing_subscriber::{prelude::*, fmt::Layer};
use crate::custom_layer::CustomLayer;
use file_rotate::ContentLimit;
use chrono_tz::Tz;


pub struct Logger;

impl Logger {
    pub fn new(path: PathBuf, limit: ContentLimit, time_zone: Tz)-> Self{
        // see if init outside of main scope is going to work:
        tracing_subscriber::registry()
            .with(
                CustomLayer{
                    path,
                    limit,
                    time_zone
                }
            )
            .init();

        Logger
    }

}