use jsonrpc_ws_server::jsonrpc_core::{IoHandler, Params, types::Error};
use jsonrpc_ws_server::ServerBuilder;

pub fn start_server() {
    let mut io = IoHandler::new();
    io.add_method("eth_getBalance", |params| {
        // let address = 
        match params {
            Params::Array(values) => values.get(0)
                .cloned()
                .ok_or_else(|| Error::invalid_params("expected 1 param - addresss.")),
            _ => Err(Error::invalid_params("expected 1 param - addresss."))
        }
        // TODO!!!: query worldstate...
    });

    let server = ServerBuilder::new(io)
        .start(&"0.0.0.0:8546".parse().unwrap())
        .expect("Server must start with no issues");

    server.wait().unwrap()
}
