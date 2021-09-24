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
use grpcio::{
    CallOption, ChannelBuilder, ClientSStreamReceiver, ClientUnaryReceiver, EnvBuilder, Metadata,
    MetadataBuilder, Result, WriteFlags,
};

fn main() {
    futures::executor::block_on(async_main()).unwrap()
}

async fn async_main() -> Result<()> {
    // Connect to server.
    env_logger::init();
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect(&format!("localhost:{}", PORT));
    let client = GreeterClient::new(ch);
    info!("Connected\n");

    // unary(&client).await?;
    // client_streaming(&client).await?;
    // server_streaming(&client).await?;
    duplex(&client).await?;

    Ok(())
}

async fn unary(client: &GreeterClient) -> Result<()> {
    // Send a single call.
    println!("\nunary\n");
    let mut builder = MetadataBuilder::with_capacity(3);
    builder.add_str("key", "client-header").unwrap();
    let headers = builder.build();

    let call_opt = CallOption::default().headers(headers);

    let mut req = HelloRequest::default();
    req.set_name("world".to_owned());
    let mut receiver: ClientUnaryReceiver<HelloReply> = client
        .say_hello_async_opt(&req, call_opt.clone())
        .expect("rpc");

    let server_headers: &Metadata = receiver.headers().await.unwrap();
    dbg!(server_headers);

    let reply: HelloReply = receiver.message().await?;
    info!("Greeter received: {}", reply.get_message());

    Ok(())
}

async fn client_streaming(client: &GreeterClient) -> Result<()> {
    // Send a list of names as a stream.
    println!("\nclient_streaming\n");

    let mut builder = MetadataBuilder::with_capacity(3);
    builder.add_str("key", "client-header").unwrap();
    let metadata = builder.build();
    let call_opt = CallOption::default().headers(metadata);

    let (mut sink, mut receiver) = client.multi_hello_opt(call_opt.clone())?;
    for name in vec!["Alice", "Bob", "Carol"] {
        let mut req = HelloRequest::default();
        req.set_name(name.to_owned());
        info!("Sending \"{}\".", name);
        sink.send((req.to_owned(), WriteFlags::default())).await?;
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    sink.close().await?;

    let server_headers: &Metadata = receiver.headers().await.unwrap();
    dbg!(server_headers);

    let reply: HelloReply = receiver.message().await?;
    info!("Multi-greeter received: {}", reply.get_message());

    Ok(())
}


async fn server_streaming(client: &GreeterClient) -> Result<()> {
    // Send a name, get several greetings back.
    println!("\nserver_streaming\n");

    let mut builder = MetadataBuilder::with_capacity(3);
    builder.add_str("key", "client-header").unwrap();
    let metadata = builder.build();
    let call_opt = CallOption::default().headers(metadata);

    let mut req = HelloRequest::default();
    req.set_name("Jeff".to_string());

    let mut receiver: ClientSStreamReceiver<HelloReply> = client.multi_reply_opt(&req, call_opt).unwrap();

    let headers = receiver.headers().await?;
    dbg!(headers);

    while let Some(reply) = receiver.try_next().await? {
        println!("C");
        info!("Got: {}", reply.get_message());
    }

    Ok(())
}


async fn duplex(client: &GreeterClient) -> Result<()> {
    println!("\nduplex\n");

    let mut builder = MetadataBuilder::with_capacity(3);
    builder.add_str("key", "client-header").unwrap();
    let metadata = builder.build();
    let call_opt = CallOption::default().headers(metadata);

    let (mut sink, mut receiver) = client.duplex_hello_opt(call_opt)?;

    let headers = receiver.headers().await?;
    dbg!(headers);

    let send = async move {
        for name in vec!["Alice", "Bob", "Carol"] {
            let mut req = HelloRequest::default();
            req.set_name(name.to_owned());
            info!("Sending \"{}\".", name);
            sink.send((req.to_owned(), WriteFlags::default())).await?;
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        sink.close().await?;
        Ok(()) as Result<_>
    };
    let receive = async move {
        while let Some(reply) = receiver.try_next().await? {
            info!("{}", reply.get_message());
        }
        Ok(()) as Result<_>
    };
    let (sr, rr) = futures::join!(send, receive);
    sr.and(rr)?;
    Ok(())
}
