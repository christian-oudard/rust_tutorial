use log::{error, info};
use std::fmt::Write;
use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use futures::channel::oneshot;
use futures::executor::block_on;
use futures::prelude::*;

use grpc_tutorial::{
    config::PORT,
    protos::{
        helloworld::{HelloReply, HelloRequest},
        helloworld_grpc::{create_greeter, Greeter},
    },
};
use grpcio::{
    DuplexSink, ChannelBuilder, ClientStreamingSink, Environment, MetadataBuilder, RequestStream,
    ResourceQuota, RpcContext, ServerBuilder, ServerStreamingSink, UnarySink, WriteFlags
};

#[derive(Clone)]
struct GreeterService;

impl Greeter for GreeterService {
    fn say_hello(&mut self, ctx: RpcContext<'_>, req: HelloRequest, sink: UnarySink<HelloReply>) {
        info!("say_hello");
        // Show metadata.
        dbg!(ctx.request_headers());

        info!("Received: {:?}", req);
        let msg = format!("Hello {}", req.get_name());
        let mut resp = HelloReply::default();
        resp.set_message(msg);

        let mut builder = MetadataBuilder::new();
        builder.add_str("key", "server-header").unwrap();

        let f = sink
            .set_headers(builder.build())
            .success(resp)
            .map_err(move |e| error!("failed to reply {:?}: {:?}", req, e))
            .map(|_| ());
        ctx.spawn(f)
    }

    fn multi_hello(
        &mut self,
        ctx: RpcContext<'_>,
        mut stream: RequestStream<HelloRequest>,
        sink: ClientStreamingSink<HelloReply>,
    ) {
        info!("multi_hello");
        // Show metadata from client.
        dbg!(ctx.request_headers());

        // Send metadata to client.
        let mut builder = MetadataBuilder::with_capacity(3);
        builder.add_str("key", "server-header").unwrap();
        let headers = builder.build();

        let f = async move {
            // Collect names from stream.
            let mut names: Vec<String> = Vec::new();
            while let Some(req) = stream.try_next().await? {
                info!("Received: {:?}", req);
                names.push(req.get_name().to_owned());
            }

            // Assemble names into message.
            let mut msg = String::new();
            if names.len() == 0 {
                write!(&mut msg, "Hello, Anonymous!").unwrap();
            } else if names.len() == 1 {
                write!(&mut msg, "Hello, {}!", names.first().unwrap()).unwrap();
            } else {
                write!(&mut msg, "Hello, ").unwrap();
                for name in names[0..names.len() - 1].iter() {
                    write!(&mut msg, "{}, ", name).unwrap();
                }
                write!(&mut msg, "and {}!", names.last().unwrap()).unwrap();
            }

            let mut resp = HelloReply::default();
            resp.set_message(msg.to_owned());
            sink
                .set_headers(headers)
                .success(resp).await?;
            Ok(())
        }
        .map_err(|e: grpcio::Error| error!("failed to reply: {:?}", e))
        .map(|_| ());
        ctx.spawn(f)
    }

    fn multi_reply(
        &mut self,
        ctx: RpcContext<'_>,
        req: HelloRequest,
        mut sink: ServerStreamingSink<HelloReply>,
    ) {
        info!("multi_reply");

        dbg!(ctx.request_headers());

        let mut builder = MetadataBuilder::with_capacity(3);
        builder.add_str("key", "server-header").unwrap();
        let metadata = builder.build();
        sink.set_headers(metadata);

        let name = req.get_name();
        info!("Received \"{}\"", name);

        let msgs = vec![
            format!("Good morning, {}.", name),
            format!("Good afternoon, {}.", name),
            format!("Goodnight, {}.", name),
        ];

        let f = async move {
            for msg in msgs {
                let mut resp = HelloReply::default();
                resp.set_message(msg);
                sink.send((resp, WriteFlags::default())).await?;
            }
            sink.close().await?;
            Ok(())
        }
        .map_err(|e: grpcio::Error| error!("failed to reply: {:?}", e))
        .map(|_| ());
        ctx.spawn(f)
    }

    fn duplex_hello(
        &mut self,
        ctx: RpcContext<'_>,
        mut stream: RequestStream<HelloRequest>,
        mut sink: DuplexSink<HelloReply>,
    ) {
        info!("duplex_hello");

        dbg!(ctx.request_headers());

        let mut builder = MetadataBuilder::with_capacity(3);
        builder.add_str("key", "server-header").unwrap();
        let metadata = builder.build();
        sink.set_headers(metadata);

        let f = async move {
            let mut resp = HelloReply::default();
            resp.set_message("Hi, what are your names?".to_string());
            sink.send((resp, WriteFlags::default())).await?;

            while let Some(req) = stream.try_next().await? {
                let msg = format!("Hello {}", req.get_name());
                let mut resp = HelloReply::default();
                resp.set_message(msg);
                sink.send((resp, WriteFlags::default())).await?;
            }

            let mut resp = HelloReply::default();
            resp.set_message("Goodbye".to_string());
            sink.send((resp, WriteFlags::default())).await?;

            sink.close().await?;
            Ok(())
        }
        .map_err(|e: grpcio::Error| error!("failed to reply: {:?}", e))
        .map(|_| ());
        ctx.spawn(f) } }

fn main() {
    env_logger::init();
    let env = Arc::new(Environment::new(1));
    let service = create_greeter(GreeterService);

    let quota = ResourceQuota::new(Some("HelloServerQuota")).resize_memory(1024 * 1024);
    let ch_builder = ChannelBuilder::new(env.clone()).set_resource_quota(quota);

    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", PORT)
        .channel_args(ch_builder.build_args())
        .build()
        .unwrap();
    server.start();
    for (host, port) in server.bind_addrs() {
        info!("listening on {}:{}", host, port);
    }
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        info!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = block_on(rx);
    let _ = block_on(server.shutdown());
}
