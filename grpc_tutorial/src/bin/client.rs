// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use log::info;

use futures::prelude::*;
use std::sync::Arc;

use grpc_tutorial::{
    config::PORT,
    protos::{
        helloworld::{HelloReply, HelloRequest},
        helloworld_grpc::GreeterClient,
    },
};
use grpcio::{CallOption, ChannelBuilder, EnvBuilder, MetadataBuilder, Result, WriteFlags, ClientUnaryReceiver};

async fn async_main() -> Result<()> {
    // Connect to server.
    env_logger::init();
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect(&format!("localhost:{}", PORT));
    let client = GreeterClient::new(ch);

    // Construct Metadata.
    let mut builder = MetadataBuilder::with_capacity(3);
    builder.add_str("k1", "v1").unwrap();
    let metadata = builder.build();
    let call_opt = CallOption::default().headers(metadata);

    // Send a single call.
    let mut req = HelloRequest::default();
    req.set_name("world".to_owned());
    let receiver: ClientUnaryReceiver<HelloReply> = client.say_hello_async_opt(&req, call_opt.clone()).expect("rpc");

    let server_metadata = receiver.headers().await;
    info!("Received headers:");
    for (key, val) in &server_metadata {
        info!("{}: {}", key, std::str::from_utf8(val).unwrap());
    }

    let reply: HelloReply = receiver.await?;
    info!("Greeter received: {}", reply.get_message());

    // Send a list of names as a stream.
    let (mut sink, receiver) = client.multi_hello_opt(call_opt.clone())?;
    for name in vec!["Alice", "Bob", "Carol"] {
        let mut req = HelloRequest::default();
        req.set_name(name.to_owned());
        info!("Sending \"{}\".", name);
        sink.send((req.to_owned(), WriteFlags::default())).await?;
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    sink.close().await?;

    // Receive reply.
    let reply: HelloReply = receiver.await?;
    info!("Multi-greeter received: {}", reply.get_message());
    Ok(())
}

fn main() {
    futures::executor::block_on(async_main()).unwrap()
}
