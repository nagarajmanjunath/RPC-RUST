extern crate futures;
extern crate ethereum_types;
extern crate ansi_term;
extern crate cid;
extern crate itertools;
extern crate multihash;
extern crate order_stat;
extern crate parking_lot;
extern crate rand;
extern crate rustc_hex;
extern crate semver;
extern crate serde;
extern crate serde_json;
extern crate tokio_timer;
extern crate transient_hashmap;


extern crate jsonrpc_core;
extern crate jsonrpc_derive;
extern crate jsonrpc_http_server as http;
extern crate jsonrpc_ipc_server as ipc;
extern crate jsonrpc_pubsub;


mod http_common;
pub use http_common::HttpMetaExtractor;
use std::net::SocketAddr;
pub mod v1;
/// RPC HTTP Server instance
pub type HttpServer = http::Server;
pub use v1::extractors::{RpcExtractor};

pub use http::{
	hyper,
	RequestMiddleware, RequestMiddlewareAction,
	AccessControlAllowOrigin, Host, DomainsValidation, cors::AccessControlAllowHeaders
};


/// Start http server asynchronously and returns result with `Server` handle on success or an error.
pub fn start_http<M, S, H, T>(
	addr: &SocketAddr,
	cors_domains: http::DomainsValidation<http::AccessControlAllowOrigin>,
	allowed_hosts: http::DomainsValidation<http::Host>,
	handler: H,
	extractor: T,
	threads: usize,
	max_payload: usize,
	keep_alive: bool,
) -> ::std::io::Result<HttpServer> where
	M: jsonrpc_core::Metadata,
	S: jsonrpc_core::Middleware<M>,
	H: Into<jsonrpc_core::MetaIoHandler<M, S>>,
	T: HttpMetaExtractor<Metadata=M>,
{
	let extractor = http_common::MetaExtractor::new(extractor);
	Ok(http::ServerBuilder::with_meta_extractor(handler, extractor)
		.keep_alive(keep_alive)
		.threads(threads)
		.cors(cors_domains)
		.allowed_hosts(allowed_hosts)
		.health_api(("/api/health", "parity_nodeStatus"))
		.cors_allow_headers(AccessControlAllowHeaders::Any)
		.max_request_body_size(max_payload * 1024 * 1024)
		.start_http(addr)?)
}