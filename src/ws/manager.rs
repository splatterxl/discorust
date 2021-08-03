use std::collections::HashMap;
use crate::ws::listeners::OpcodeListener;
use url::Url;
use tokio_tungstenite::connect_async;

#[derive(Debug)]
pub struct WebSocketManager {
  url: String,
  listeners: HashMap<u8, OpcodeListener>,
  stream: Option<String>
}

pub const EMPTY_STRING: String = String::new();
pub const GATEWAY: &'static str = "wss://gateway.discord.gg/?v=9&encoding=json";

impl WebSocketManager {
  pub fn new() -> Self {
    Self {
        url: String::from(GATEWAY),
        listeners: HashMap::new(),
        stream: None
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

            // TODO: find a way to typedef this correctly *shudders*
            //self.stream = Some(stream);

            self
        },
        Err(err) => {
            dbg!(err);

            panic!("Error connecting to gateway.");
        }
    }
  }
}
