//! Parity-specific metadata extractors.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use crate::http_common::HttpMetaExtractor;
use jsonrpc_core as core;
use jsonrpc_core::futures::future::Either;
use jsonrpc_pubsub::Session;
use crate::v1;
use v1::metadata::Metadata;
use v1::Origin;



/// Common HTTP & IPC metadata extractor.
pub struct RpcExtractor;

impl HttpMetaExtractor for RpcExtractor {
	type Metadata = Metadata;

	fn read_metadata(&self, origin: Option<String>, user_agent: Option<String>) -> Metadata {
		Metadata {
			origin: Origin::Rpc(
				format!("{} / {}",
						origin.unwrap_or_else(|| "unknown origin".to_string()),
						user_agent.unwrap_or_else(|| "unknown agent".to_string()))
			),
			session: None,
		}
	}
}