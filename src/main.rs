use jsonrpc_core::*;
use jsonrpc_http_server::*;

fn main() {
    let mut io = IoHandler::new();
    io.add_method("hello", |_: Params| {
        Ok(Value::String("Hello World!".to_string()))
    });

    let address = "127.0.0.1:3030";
    println!("Starting the server at {}", address);

    let _server = ServerBuilder::new(io)
        .start_http(&address.parse().unwrap())
        .expect("Unable to start RPC server");

    _server.wait();
}
