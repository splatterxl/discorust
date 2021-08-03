use discorust::client::Client;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let token = unsafe {
		String::from_utf8_unchecked(
			fs::read(format!("{}/token.env", env!("PWD"))).expect("Expected a token.env file."),
		)
	};
	let mut client = Client::new();

	client.set_token(&token);

	match client.connect().await {
		Ok(_) => {
			println!("Logged in!");
		}
		Err(err) => {
			println!("Oops! Something went wrong!");
			dbg!(err);
		}
	}

	Ok(())
}
