extern crate secp256k1;
extern crate rand;

use secp256k1::rand::rngs::OsRng;
use sha2::{Sha256, Digest};
use secp256k1::{All, Secp256k1, Message, SecretKey, PublicKey, Signature};
use secp256k1::bitcoin_hashes::sha256;

pub struct KeyMaster {
  pub secp: Secp256k1<All>,
  pub public_key: PublicKey,
  pub secret_key: SecretKey,
  pub rng: OsRng
}

impl KeyMaster {
  pub fn new() -> KeyMaster {
    let secp = Secp256k1::new();
    let mut rng = OsRng::new().expect("OsRng");
    let secret_key = SecretKey::new(&mut rng);
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    return KeyMaster {
      secp: secp,
      rng: rng,
      secret_key: secret_key,
      public_key: public_key
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
  return keys.secp.sign(&message_, &keys.secret_key);
}

pub fn verify(keys: &KeyMaster, message: Message, signature: Signature) -> bool {
  return keys.secp.verify(&message, &signature, &keys.public_key).is_ok();
}

pub fn hash_string(in_str: &str) -> String {
  // create a Sha256 object
  let mut hasher = Sha256::new();

  // write input message
  hasher.update(in_str);

  return format!("{:x}", hasher.finalize());
}

