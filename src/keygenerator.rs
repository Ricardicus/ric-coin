//use hex_literal::hex;
//use sha2::{Sha256, Sha512, Digest};
extern crate secp256k1;
extern crate rand;

use rand::OsRng;

use sha2::{Sha256, Digest};
use secp256k1::{Secp256k1, SecretKey, PublicKey};
//use secp256k1::bitcoin_hashes::sha256;

pub fn generate_secp_keys() -> (SecretKey, PublicKey) {
  let secp = Secp256k1::new();
  let mut rng = OsRng::new().expect("OsRng");
  let secret_key = SecretKey::new(&mut rng);
  let public_key = PublicKey::from_secret_key(&secp, &secret_key);
  return (secret_key, public_key);
}

pub fn hash_string(in_str: &str) -> String {
  // create a Sha256 object
  let mut hasher = Sha256::new();

  // write input message
  hasher.update(in_str);

  return format!("{:x}", hasher.finalize());
}
