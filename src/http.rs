use crate::types::GetGatewayBot;
use reqwest::{header::HeaderMap, Client, ClientBuilder, Response};

#[derive(Debug, Clone)]
pub struct HTTP {
	token: Option<&'static str>,
	api_version: i8,
	client: Option<Client>,
}

const BASE_URL: &'static str = "https://discord.com/api/";

impl HTTP {
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
					.get(BASE_URL.to_owned() + "v" + &self.api_version.to_string() + endpoint)
					.build();

				match req {
					Ok(request) => {
						let resp = client.execute(request).await?;
						Ok(resp)
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

	pub fn set_token(&mut self, token: &'static str) {
		self.token = Some(token);
	}

	pub fn init(&mut self) {
		match self.token {
			Some(_value) => {
				let mut headers = HeaderMap::new();
				headers.insert("Authorization", self.token.unwrap().parse().unwrap());
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
