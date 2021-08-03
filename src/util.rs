pub fn validate_token(token: &String) -> bool {
	if token.starts_with("mfa.") {
		panic!("Using user accounts in the Bot API constitutes selfbotting and is against ToS.");
	};

	let mut ind: i8 = 0;

	for _ in token.split(|c| (c == '.')) {
		ind += 1;
	}

	if ind != 3 {
		return false;
	}

	true
}

pub fn censor_token(token: &String) -> String {
	let matched = token.split(|c| (c == '.'));
	let mut arr = Vec::new();

	for part in matched {
		if arr.len() == 2 {
			arr.push("*".repeat(part.len()));
		} else {
			arr.push(part.to_string());
		}
	}

	arr.join(".")
}
