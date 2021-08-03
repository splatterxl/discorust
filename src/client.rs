use crate::http::HTTP;
use crate::util;

#[derive(Debug, Clone)]
pub struct Client {
	pub token: String,
	pub http: HTTP,
}

pub const BASE_TOKEN: &'static str = "";

impl Client {
	pub fn new() -> Self {
		Self {
			token: String::from(BASE_TOKEN),
			http: HTTP::new(9),
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

	pub async fn connect(&mut self) -> Result<&String, Box<dyn std::error::Error>> {
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
					dbg!(resp);
				}
				Err(err) => {
					panic!("Could not GET /gateway/bot. {}", err);
				}
			}

			Ok(&self.token)
		}
	}
}
