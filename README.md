# JSON RPC Server
This is a simple jsonrpc server.

## Installation
1. Get rust tooling installed. I use this command: 
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s
```
2. Clone this repository to your computer.

## Usage
1. You can run the code by `cargo run`. This will start a server and wait for
   requests. First time compiling can take a long time because you're also getting all the dependencies.
2. To send requests, you can use `curl` like this:
```
$ curl --data-binary '{"jsonrpc":"2.0","id":"curltext","method":"hello"}' -H 'content-type:application/json' http://localhost:3030
{"jsonrpc":"2.0","result":"Hello World!","id":"curltext"}
```
The `id` field is your client id, `jsonrpc` field and `content-type` is required for this jsonrpc library. The `result` field in your results show the return value of the function.
