use std::{path::PathBuf};
pub use tracing::{debug_span, instrument, info, warn, error, info_span, Subscriber};
use tracing_subscriber::{prelude::*, fmt::Layer};
use crate::custom_layer::CustomLayer;

pub struct Logger;

impl Logger {
    pub fn new(path: PathBuf)-> Self{
        // see if init outside of main scope is going to work:
        tracing_subscriber::registry()
            .with(
                CustomLayer{
                    path
                }
            )
            .init();

        Logger
    }

}