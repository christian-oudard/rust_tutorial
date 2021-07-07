// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use log::info;

use std::sync::Arc;
use futures::prelude::*;

use grpc_tutorial::{
    config::PORT,
    protos::{
        helloworld::{HelloReply, HelloRequest},
        helloworld_grpc::GreeterClient,
    },
};
use grpcio::{ChannelBuilder, EnvBuilder, Result, WriteFlags};


async fn async_main() -> Result<()> {
    env_logger::init();
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect(&format!("localhost:{}", PORT));
    let client = GreeterClient::new(ch);

    let (mut sink, receiver) = client.multi_hello()?;

    for name in vec!["Alice", "Bob", "Carol"] {
        let mut req = HelloRequest::default();
        req.set_name(name.to_owned());
        info!("Sending \"{}\".", name);
        sink.send((req.to_owned(), WriteFlags::default())).await?;
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    sink.close().await?;
    let reply: HelloReply = receiver.await?;
    info!("Greeter received: {}", reply.get_message());
    Ok(())
}

fn main() {
    futures::executor::block_on(async_main()).unwrap()
}

