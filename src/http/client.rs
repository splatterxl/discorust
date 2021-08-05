use crate::types::GetGatewayBot;
use reqwest::{header::HeaderMap, Client, ClientBuilder, Response};
use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct HTTPClient {
	token: Option<String>,
	api_version: i8,
	client: Option<Client>,
}

#[derive(Debug)]
pub struct RateLimitError<'a> {
	details: &'a str,
}

impl RateLimitError<'_> {
	pub fn new() -> Self {
		Self {
			details: "You are being rate-limited.",
		}
	}
}

impl fmt::Display for RateLimitError<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.details)
	}
}

impl Error for RateLimitError<'_> {
	fn description(&self) -> &str {
		&self.details
	}
}

const BASE_URL: &'static str = "https://discord.com/api/";

impl HTTPClient {
	pub fn new(api_version: i8) -> Self {
		Self {
			token: None,
			api_version,
			client: None,
		}
	}

	pub async fn request(&self, endpoint: &str) -> Result<Response, Box<dyn std::error::Error>> {
		match &self.client {
			Some(client) => {
				let req = client
					.get(String::from(BASE_URL) + "v" + &self.api_version.to_string() + endpoint)
					.build();

				match req {
					Ok(request) => {
						let resp = client.execute(request).await?;
						let status = resp.status();
						if status == 401 {
							panic!("An invalid token was provided to the client");
						} else if status == 429 {
							Err(Box::new(RateLimitError::new()))
						} else {
							Ok(resp)
						}
					}
					Err(err) => {
						panic!("An error occurred when building the request. {}", err);
					}
				}
			}
			None => {
				panic!("A request was attempted while the client was uninitialized.");
			}
		}
	}

	pub fn set_token(&mut self, token: String) {
		self.token = Some(token);
	}

	pub fn init(&mut self) {
		match &self.token {
			Some(_value) => {
				let mut headers = HeaderMap::new();
				headers.insert(
					"Authorization",
					(self.token.clone().unwrap()).parse().unwrap(),
				);
				let builder = ClientBuilder::new().default_headers(headers);
				match builder.build() {
					Ok(client) => {
						self.client = Some(client);
					}
					Err(err) => {
						panic!("An error occurred when building a HTTP client. {}", err)
					}
				}
			}
			None => {
				panic!("Attempted to initialize HTTP manager with no token provided.");
			}
		}
	}

	pub async fn get_gateway_bot(&self) -> Result<GetGatewayBot, Box<dyn std::error::Error>> {
		let resp = self
			.request("/gateway/bot")
			.await?
			.json::<GetGatewayBot>()
			.await?;

		Ok(resp)
	}
}
