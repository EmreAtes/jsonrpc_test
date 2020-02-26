// Import the packages
use jsonrpc_core::*;
use jsonrpc_http_server::*;

/// This is the function that will be called
fn hello(_: Params) -> Result<Value> {
    Ok(Value::String("Hello World!".to_string()))
}

fn main() {
    // Construct the IOHandler object and add the functions
    let mut io = IoHandler::new();
    io.add_method("hello", hello);

    let address = "127.0.0.1:3030";
    println!("Starting the server at {}", address);

    // Start the server
    let _server = ServerBuilder::new(io)
        .start_http(&address.parse().unwrap())
        .expect("Unable to start RPC server");

    _server.wait();
}
