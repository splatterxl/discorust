use crate::{
	types::{GatewayDispatch, HelloDispatch, IdentifyDispatch, Opcodes},
	ws::listeners::OpcodeListener,
};
use futures_util::{
	stream::{SplitSink, SplitStream},
	SinkExt, StreamExt,
};
use serde_json::{from_str, to_string};
use std::{collections::HashMap, error::Error};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use url::Url;

#[derive(Debug)]
pub struct WebSocketManager {
	url: String,
	listeners: HashMap<u8, OpcodeListener>,
	heartbeat_interval: u32,
	write: Option<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>,
	read: Option<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
	token: String,
}

pub const EMPTY_STRING: String = String::new();
pub const GATEWAY: &'static str = "wss://gateway.discord.gg/?v=9&encoding=json";

impl WebSocketManager {
	pub fn new() -> Self {
		Self {
			url: String::from(GATEWAY),
			listeners: HashMap::new(),
			heartbeat_interval: 0,
			write: None,
			read: None,
			token: EMPTY_STRING,
		}
	}

	pub async fn connect(&mut self, token: &String, url: String) {
		let url = url + "/?v=9&encoding=json";
		if &url != &self.url {
			self.url = url;
		}
		self.token = token.to_string();
		self.url = String::from(&self.url);
		self.debug(&format!("Connecting to gateway... [URL: {}]", self.url));
		let url = Url::parse(&self.url).unwrap();

		let connection = connect_async(&url).await;

		match connection {
			Ok(val) => {
				let (stream, _) = val;
				self.debug("WebSocket handshake completed.");

				let (mut write, mut read) = stream.split();

				while let Some(Ok(msg)) = read.next().await {
					match msg {
						Message::Text(payload) => {
							self.debug(&format!("payload = {:#?}", &payload));
							let json: GatewayDispatch<HelloDispatch> = from_str(payload.as_str())
								.expect("Payload was not correctly formatted");
							match json.op {
								0_u8..=9_u8 => todo!(),
								10 => {
									let json = from_str::<GatewayDispatch<HelloDispatch>>(
										payload.as_str(),
									)
									.unwrap();

									let interval = json.d.unwrap().heartbeat_interval;
									self.debug(&format!("Set heartbeat interval to {}", &interval));
									self.set_heartbeat_interval(interval);

									self.identify(&mut write, (0, 1))
										.await
										.expect("Couldn't IDENTIFY");

									self.debug("Sent IDENTIFY");
								}
								11 => todo!(),
								// rust dies if I don't do this :weary:
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
									reason = String::from("None");
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
			Err(err) => {
				dbg!(err);

				panic!("Error connecting to gateway.");
			}
		}
	}

	pub async fn identify(
		&self,
		stream: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
		_shards: (u32, u32),
	) -> Result<(), Box<dyn Error>> {
		let token = &self.token;
		let dispatch: GatewayDispatch<IdentifyDispatch> = GatewayDispatch {
			op: Opcodes::IDENTIFY,
			s: None,
			d: Some(IdentifyDispatch {
				token: token.to_owned(),
			}),
			t: None,
		};

		let string = to_string(&dispatch).expect("Couldn't parse IDENTIFY dispatch");

		stream.send(Message::Text(string)).await?;

		Ok(())
	}

	fn set_heartbeat_interval(&mut self, interval: u32) {
		self.heartbeat_interval = interval;
	}

	fn debug(&mut self, message: &str) {
		println!("[WS] {}", message)
	}
}
