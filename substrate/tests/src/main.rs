#[macro_use]
extern crate jsonrpc_core_client;
extern crate futures;
extern crate substrate_rpc;
extern crate node_primitives;
extern crate hyper;

use hyper::rt;
use jsonrpc_core_client::{
    transports::http,
    transports::ws,
    RpcError,
    TypedClient,
    RpcChannel,
};
use futures::Future;
use substrate_rpc::system::SystemClient;
use node_primitives::Hash;

use substrate_rpc::author::{
    AuthorClient,
    hash::ExtrinsicOrHash,
};

fn main() {

    rt::run(rt::lazy(|| {
        let uri = "http://192.168.2.158:9933";

        http::connect(uri)
            .and_then(|client: SystemClient<Hash, Hash>| {
                get_system_name(client)
            })
            .map_err(|e| {
                println!("Error: {:?}", e);
            })
    }))
}

fn get_system_name(client: SystemClient<Hash, Hash>) -> impl Future<Item=(), Error=RpcError> {
    client.system_name()
        .map(|removed| {
            println!("{:?}", removed);
        })
}

