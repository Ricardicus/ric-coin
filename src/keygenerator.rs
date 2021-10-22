//use hex_literal::hex;
//use sha2::{Sha256, Sha512, Digest};
use sha2::{Sha256, Digest};

pub fn hash_string(in_str: &str) -> String {
  // create a Sha256 object
  let mut hasher = Sha256::new();

  // write input message
  hasher.update(in_str);

  return format!("{:x}", hasher.finalize());
}
