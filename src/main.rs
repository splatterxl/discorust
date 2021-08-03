use discorust::client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut client = Client::new();
	client.set_token("uwu.owo.ewe");

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
