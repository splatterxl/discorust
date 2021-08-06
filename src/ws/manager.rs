use crate::{
	types::{GatewayDispatch, HelloDispatch, IdentifyDispatch, Opcodes},
	ws::listeners::OpcodeListener,
};
use futures_util::{
	stream::{SplitSink, SplitStream},
	StreamExt,
};
use serde_json::{from_str, to_string};
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
			stream: None,
			heartbeat_interval: 0,
			write: None,
			read: None,
			token: EMPTY_STRING,
		}
	}

	pub async fn connect(mut self, token: String, url: String) {
		let url = url + "/?v=9&encoding=json";
		if &url != &self.url {
			self.url = url;
		}
		self.token = token;
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
			}
			Err(err) => {
				dbg!(err);

				panic!("Error connecting to gateway.");
			}
		}
	}

	async fn listen(mut self) {
		let stream = self.stream.as_mut().unwrap();

		let (mut write, mut read) = stream.split();

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
							let interval = json.d.unwrap().heartbeat_interval;
							self.set_heartbeat_interval(&interval);
							println!("Set heartbeat interval to {}", &interval);
							self.identify(write);
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

	pub fn identify(
		mut self,
		stream: SplitSink<&mut WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
	) {
		let token = self.token;
		let dispatch: GatewayDispatch<IdentifyDispatch> = GatewayDispatch {
			op: Opcodes::IDENTIFY,
			s: None,
			d: Some(IdentifyDispatch { token }),
			t: None,
		};
	}

	fn set_heartbeat_interval(mut self, interval: &u32) {
		self.heartbeat_interval = interval;
	}
}
