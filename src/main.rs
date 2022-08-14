use std::{path::PathBuf};

use tracing::{instrument, info, warn, error};

mod logger;
use logger::Logger;
mod custom_layer;
use custom_layer::CustomLayer;
use file_rotate::ContentLimit;


fn main() {
   let logger = Logger::new(PathBuf::from(r"logs"), ContentLimit::Bytes(1024));//tracing_subscriber::registry().with(CustomLayer).init();
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
    function_c();
    warn!("Exiting function b");

}

#[instrument]
fn function_c(){
    info!("Inside function c");
    error!("Exiting function c");
}