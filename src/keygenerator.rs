extern crate secp256k1;
extern crate rand;

use secp256k1::rand::rngs::OsRng;
use sha2::{Sha256, Digest};
use secp256k1::{All, Secp256k1, Message, SecretKey, PublicKey, Signature};
use secp256k1::bitcoin_hashes::sha256;
use std::str::FromStr;

pub struct KeyMaster {
  pub secp: Secp256k1<All>,
  pub public_key: String,
  pub secret_key: String,
}

impl KeyMaster {
  pub fn new() -> KeyMaster {
    let secp = Secp256k1::new();
    let mut rng = OsRng::new().expect("OsRng");
    let secret_key = SecretKey::new(&mut rng);
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    /*let secret_key_str = secret_key.to_string();
    let secret_key_from_str = SecretKey::from_str(&secret_key_str[..])
        .expect("Failed");
    let secret_key_str_two = secret_key_from_str.to_string();
    println!("secret_key_to_str: {}", secret_key_str);
    println!("secret_key_back:   {}", secret_key_str_two);*/
    return KeyMaster {
      secp: secp,
      secret_key: secret_key.to_string(),
      public_key: public_key.to_string()
    };
  }
}

pub fn generate_secp_keys() -> (SecretKey, PublicKey) {
  let secp = Secp256k1::new();
  let mut rng = OsRng::new().expect("OsRng");
  let secret_key = SecretKey::new(&mut rng);
  let public_key = PublicKey::from_secret_key(&secp, &secret_key);
  return (secret_key, public_key);
}

pub fn sign(keys: &KeyMaster, message: String) -> Signature { 
  let message_ = Message::from_hashed_data::<sha256::Hash>(message.as_bytes());
  return keys.secp.sign(&message_, &SecretKey::from_str(&keys.secret_key[..]).unwrap());
}

pub fn verify(keys: &KeyMaster, message: String, signature: Signature) -> bool {
  let message_ = Message::from_hashed_data::<sha256::Hash>(message.as_bytes());
  return keys.secp.verify(&message_, &signature, &PublicKey::from_str(&keys.public_key[..]).unwrap()).is_ok();
}

pub fn hash_string(in_str: &str) -> String {
  // create a Sha256 object
  let mut hasher = Sha256::new();

  // write input message
  hasher.update(in_str);

  return format!("{:x}", hasher.finalize());
}

