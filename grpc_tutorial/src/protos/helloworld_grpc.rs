// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_GREETER_SAY_HELLO: ::grpcio::Method<super::helloworld::HelloRequest, super::helloworld::HelloReply> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/helloworld.Greeter/SayHello",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_GREETER_MULTI_HELLO: ::grpcio::Method<super::helloworld::HelloRequest, super::helloworld::HelloReply> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ClientStreaming,
    name: "/helloworld.Greeter/MultiHello",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_GREETER_MULTI_REPLY: ::grpcio::Method<super::helloworld::HelloRequest, super::helloworld::HelloReply> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/helloworld.Greeter/MultiReply",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_GREETER_DUPLEX_HELLO: ::grpcio::Method<super::helloworld::HelloRequest, super::helloworld::HelloReply> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/helloworld.Greeter/DuplexHello",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct GreeterClient {
    client: ::grpcio::Client,
}

impl GreeterClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        GreeterClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn say_hello_opt(&self, req: &super::helloworld::HelloRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::helloworld::HelloReply> {
        self.client.unary_call(&METHOD_GREETER_SAY_HELLO, req, opt)
    }

    pub fn say_hello(&self, req: &super::helloworld::HelloRequest) -> ::grpcio::Result<super::helloworld::HelloReply> {
        self.say_hello_opt(req, ::grpcio::CallOption::default())
    }

    pub fn say_hello_async_opt(&self, req: &super::helloworld::HelloRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::helloworld::HelloReply>> {
        self.client.unary_call_async(&METHOD_GREETER_SAY_HELLO, req, opt)
    }

    pub fn say_hello_async(&self, req: &super::helloworld::HelloRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::helloworld::HelloReply>> {
        self.say_hello_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn multi_hello_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::helloworld::HelloRequest>, ::grpcio::ClientCStreamReceiver<super::helloworld::HelloReply>)> {
        self.client.client_streaming(&METHOD_GREETER_MULTI_HELLO, opt)
    }

    pub fn multi_hello(&self) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::helloworld::HelloRequest>, ::grpcio::ClientCStreamReceiver<super::helloworld::HelloReply>)> {
        self.multi_hello_opt(::grpcio::CallOption::default())
    }

    pub fn multi_reply_opt(&self, req: &super::helloworld::HelloRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::helloworld::HelloReply>> {
        self.client.server_streaming(&METHOD_GREETER_MULTI_REPLY, req, opt)
    }

    pub fn multi_reply(&self, req: &super::helloworld::HelloRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::helloworld::HelloReply>> {
        self.multi_reply_opt(req, ::grpcio::CallOption::default())
    }

    pub fn duplex_hello_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::helloworld::HelloRequest>, ::grpcio::ClientDuplexReceiver<super::helloworld::HelloReply>)> {
        self.client.duplex_streaming(&METHOD_GREETER_DUPLEX_HELLO, opt)
    }

    pub fn duplex_hello(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::helloworld::HelloRequest>, ::grpcio::ClientDuplexReceiver<super::helloworld::HelloReply>)> {
        self.duplex_hello_opt(::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Greeter {
    fn say_hello(&mut self, ctx: ::grpcio::RpcContext, req: super::helloworld::HelloRequest, sink: ::grpcio::UnarySink<super::helloworld::HelloReply>);
    fn multi_hello(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::helloworld::HelloRequest>, sink: ::grpcio::ClientStreamingSink<super::helloworld::HelloReply>);
    fn multi_reply(&mut self, ctx: ::grpcio::RpcContext, req: super::helloworld::HelloRequest, sink: ::grpcio::ServerStreamingSink<super::helloworld::HelloReply>);
    fn duplex_hello(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::helloworld::HelloRequest>, sink: ::grpcio::DuplexSink<super::helloworld::HelloReply>);
}

pub fn create_greeter<S: Greeter + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_GREETER_SAY_HELLO, move |ctx, req, resp| {
        instance.say_hello(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_client_streaming_handler(&METHOD_GREETER_MULTI_HELLO, move |ctx, req, resp| {
        instance.multi_hello(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_GREETER_MULTI_REPLY, move |ctx, req, resp| {
        instance.multi_reply(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_duplex_streaming_handler(&METHOD_GREETER_DUPLEX_HELLO, move |ctx, req, resp| {
        instance.duplex_hello(ctx, req, resp)
    });
    builder.build()
}
