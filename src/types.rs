use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct GetGatewayBot {
	pub url: String,
	pub shards: i32,
	pub session_start_limit: GatewaySessionStartLimit,
}

#[derive(Deserialize, Debug)]
pub struct GatewaySessionStartLimit {
	pub total: i32,
	pub remaining: i32,
	pub reset_after: f64,
	pub max_concurrency: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GatewayDispatch<Data> {
	pub op: u8,
	pub s: Option<u64>,
	pub d: Option<Data>,
	pub t: Option<String>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct HelloDispatch {
	pub heartbeat_interval: u32,
}

pub mod Opcodes;

pub enum GatewayCloseCodes {
	UnknownError = 1000,
	UnknownOpcode,
	DecodeError,
	NotAuthenticated,
	AuthenticationFailed,
	AlreadyAuthenticated,
	InvalidSeq,
	RateLimit,
	SessionTimedOut,
	InvalidShard,
	ShardingRequired,
	InvalidAPIVersion,
	InvalidIntents,
	DisallowedIntents,
}

#[derive(Deserialize, Serialize)]
pub struct IdentifyDispatch {
    token: String
}
