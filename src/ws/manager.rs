use crate::{
	types::{GatewayDispatch, HelloDispatch},
	ws::listeners::OpcodeListener,
};
use futures_util::StreamExt;
use serde_json::from_str;
use std::collections::HashMap;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use url::Url;

#[derive(Debug)]
pub struct WebSocketManager {
	url: String,
	listeners: HashMap<u8, OpcodeListener>,
	stream: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
	heartbeat_interval: u32,
}

pub const EMPTY_STRING: String = String::new();
pub const GATEWAY: &'static str = "wss://gateway.discord.gg/?v=9&encoding=json";

impl WebSocketManager {
	pub fn new() -> Self {
		Self {
			url: String::from(GATEWAY),
			listeners: HashMap::new(),
			stream: None,
			heartbeat_interval: 0,
		}
	}

	pub async fn connect(&mut self, url: String) -> &mut Self {
		let url = url + "/?v=9&encoding=json";
		if &url != &self.url {
			self.url = url;
		}
		self.url = String::from(&self.url);
		println!("Connecting to gateway... [URL: {}]", self.url);
		let url = Url::parse(&self.url).unwrap();

		let connection = connect_async(&url).await;

		match connection {
			Ok(val) => {
				let (stream, _) = val;
				println!("WebSocket handshake completed.");

				self.stream = Some(stream);

				self.listen().await;

				self
			}
			Err(err) => {
				dbg!(err);

				panic!("Error connecting to gateway.");
			}
		}
	}

	async fn listen(&mut self) {
		let stream = self.stream.as_mut().unwrap();

		let (_, mut read) = stream.split();

		while let Some(Ok(msg)) = read.next().await {
			match msg {
				Message::Text(payload) => {
					dbg!(&payload);
					let json: GatewayDispatch<HelloDispatch> =
						from_str(payload.as_str()).expect("Payload was not correctly formatted");
					match json.op {
						0_u8..=9_u8 => todo!(),
						10 => {
							let json = from_str::<GatewayDispatch<HelloDispatch>>(payload.as_str())
								.unwrap();
							self.heartbeat_interval = json.d.unwrap().heartbeat_interval;
							println!("Set heartbeat interval to {}", self.heartbeat_interval);
						}
						11 => todo!(),
						// idk
						12_u8..=u8::MAX => {}
					}
				}
				Message::Binary(_) => todo!(),
				Message::Ping(_) => {}
				Message::Pong(_) => {}
				Message::Close(frame) => {
					if frame.is_none() {
						panic!("ok");
					} else {
						let frame = frame.unwrap();

						let mut reason = frame.reason.to_string();

						if reason == "" {
							reason = String::from("none");
						}

						panic!(
							"Gateway connection closed with code {} (reason {})",
							frame.code, reason
						)
					}
				}
			}
		}
	}
}
