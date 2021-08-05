use serde::Deserialize;

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

#[derive(Deserialize, Debug)]
pub struct GatewayDispatch<Data> {
	pub op: u8,
	pub s: Option<u64>,
	pub d: Option<Data>,
	pub t: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct HelloDispatch {
	pub heartbeat_interval: u32,
}

#[derive(Deserialize, Debug)]
pub enum Opcodes {
	Dispatch,
	Heartbeat,
	Identify,
	PresenceUpdate,
	VoiceStateUpdate,
	Resume,
	Reconnect,
	RequestGuildMembers,
	InvalidSession,
	Hello,
	HeartbeatAck,
}

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
