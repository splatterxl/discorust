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
