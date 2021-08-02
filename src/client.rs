use crate::util;

#[derive(Debug)]
pub struct Client<'a> {
  pub token: &'a str
}

const BASE_TOKEN: &'static str = "";

impl Client<'_> {
  pub fn new() -> Self {
    Self {
      token: BASE_TOKEN
    }
  }

  pub fn set_token(mut self, token: &'static str) -> Self {
    if util::validate_token(&String::from(token)) {
      self.token = token;
      self
    } else {
      panic!("Invalid token provided.");
    }
  }

  pub fn connect(&mut self) {
    if self.token == BASE_TOKEN {
      panic!("No token provided for the client.");
    } else {
      println!("Logging in with token {}...", util::censor_token(self.token));
    }
  }
}
