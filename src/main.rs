use std::{cell::Cell, path::PathBuf};

use tracing::{debug_span, instrument, info, warn, error, info_span, Subscriber};
use tracing_subscriber::{prelude::*, fmt::Layer};

mod logger;
use logger::Logger;
mod custom_layer;
use custom_layer::CustomLayer;
use file_rotate::ContentLimit;


fn main() {
   let logger = Logger::new(PathBuf::from(r"logs"), ContentLimit::Bytes(1024));//tracing_subscriber::registry().with(CustomLayer).init();
    function_a();
}
#[instrument(level = "trace", target = "collection_loop")]
fn function_a(){
    info!(message = "Inside function a", path = "Transactions", level = "error");
    function_b();

    warn!("Exiting function a");
}

#[instrument(level = "debug", target = "making_request")]
fn function_b(){
    info!("Inside function b");
    function_c();
    warn!("Exiting function b");

}

#[instrument(target = "checking_threads")]
fn function_c(){
    info!("Inside function c");
    error!("Exiting function c");
}