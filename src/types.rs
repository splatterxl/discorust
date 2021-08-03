use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GetGatewayBot {
	url: String,
	shards: i32,
	session_start_limit: GatewaySessionStartLimit,
}

#[derive(Deserialize, Debug)]
pub struct GatewaySessionStartLimit {
	total: i32,
	remaining: i32,
	reset_after: f64,
	max_concurrency: i32,
}
