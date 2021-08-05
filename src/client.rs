use crate::{http::client::HTTPClient, util, ws::manager::WebSocketManager};
#[derive(Debug)]
pub struct Client {
	pub token: String,
	pub http: HTTPClient,
	pub ws: WebSocketManager,
}

pub const BASE_TOKEN: &'static str = "";

impl Client {
	pub fn new() -> Self {
		Self {
			token: String::from(BASE_TOKEN),
			http: HTTPClient::new(9),
			ws: WebSocketManager::new(),
		}
	}

	pub fn set_token(&mut self, token: &String) {
		if util::validate_token(token) {
			let token: String = String::from(format!("Bot {}", token));
			self.token = token;
		} else {
			panic!("Invalid token provided.");
		}
	}

	pub async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
		if self.token == BASE_TOKEN {
			panic!("No token provided for the client.");
		} else {
			self.http.set_token(self.token.clone());
			self.http.init();
			println!(
				"Logging in with token {}...",
				util::censor_token(&self.token)
			);
			match self.http.get_gateway_bot().await {
				Ok(resp) => {
					if resp.shards > 1 {
						panic!("discorust does not support sharding at the current time.");
					}
					self.ws.connect(&self.token, resp.url).await;
				}
				Err(err) => return Err(err),
			}
		}

        Ok(())
	}
}
