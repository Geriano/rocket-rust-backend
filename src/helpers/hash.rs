use sha2::Digest;
use sha2::Sha512;

#[derive(Clone)]
pub struct Hash {
  messages: Vec<String>,
  hex: Option<String>,
}

impl Hash {
  pub fn new(salt: String) -> Hash {
    Hash {
      messages: vec![salt],
      hex: None,
    }
  }

  pub fn append(&mut self, message: String) -> &mut Self {
    self.messages.push(message);

    self
  }

  pub fn digest(&mut self) -> String {
    if self.hex.is_none() {
      let mut hasher = Sha512::new();
      
      self.messages
        .iter()
        .for_each(|message| hasher.update(message.as_bytes()));
  
      let result = Some(hex::encode(
        hasher.finalize()
      ));

      self.hex = result;
    }

    self.hex.clone().unwrap()
  }
}

