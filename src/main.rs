mod logger;
mod custom_layer;
mod write_json;
//use file_rotate::ContentLimit;
use tracing_storage_logger::prelude::*;
use serde_json::json;

fn main() {
    // It's going to complain this is an unused variable. That's fine. Use an "_" underscore if you wish:
    let logger = Logger::new(
        PathBuf::from(r"logs"), 
        ContentLimit::Bytes(1_024_000),
        4,
        chrono_tz::US::Eastern
    );
   

    info!(message = "🍺🍺🍺 Cheers!", path = "general");
    info!(message = "🌈🌈🌈 Peace and beauty", path = "general");//tracing_subscriber::registry().with(CustomLayer).init();
    function_a();
}

#[instrument(level = "trace")]
fn function_a(){
    let value = json!({
        "jsonrpc": "2.0",
        "method": "eth_estimateGas",
        "params": [
            {
                "from": "0xb60e8dd61c5d32be8058bb8eb970870f07233155",
                "to": "0xd46e8dd67c5d32be8058bb8eb970870f07244567",
                "gas": "0x76c0",
                "gasPrice": "0x9184e72a000",
                "value": "0x9184e72a",
                "data": "0xd46e8dd67c5d32be8d46e8dd67c5d32be8058bb8eb970870f072445675058bb8eb970870f072445675",
                "info":{"name":"Me","date":"Yesterday"}
            },
        ],
        "id": 1,
       
    });
    info!(json = value.to_string());
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