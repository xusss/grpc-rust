#[macro_use]
extern crate log;
#[macro_use]
extern crate futures;
extern crate bytes;
extern crate futures_cpupool;
extern crate tokio_core;
extern crate tokio_tls;
extern crate base64;

extern crate protobuf;

extern crate httpbis;

pub mod client;
pub mod server;

mod grpc;
mod grpc_frame;
mod grpc_http_to_response;

pub mod method;
pub mod grpc_protobuf;
pub mod marshall;
pub mod futures_grpc;
pub mod result;
pub mod error;
pub mod iter;
pub mod rt;
pub mod metadata;
mod stream_item;
mod req;
mod resp;
mod chars;


pub use stream_item::GrpcItemOrMetadata;
pub use resp::GrpcSingleResponse;
pub use resp::GrpcStreamingResponse;
pub use req::GrpcRequestOptions;
pub use req::GrpcStreamingRequest;

pub use metadata::GrpcMetadata;


pub mod for_test {
    pub use httpbis::server_conn::*;
    pub use httpbis::client_conn::*;
    pub use httpbis::http_common::*;
    pub use httpbis::solicit_async::*;
    pub use httpbis::futures_misc::*;
}
