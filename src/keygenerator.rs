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
  pub secret_key: String
}

/* Keymaster holds the keys for the transactions */
impl KeyMaster {
  pub fn new() -> KeyMaster {
    let secp = Secp256k1::new();
    let mut rng = OsRng::new().expect("OsRng");
    let secret_key = SecretKey::new(&mut rng);
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    return KeyMaster {
      secp: secp,
      secret_key: secret_key.to_string(),
      public_key: public_key.to_string()
    };
  }

  /* To start it from already generated values */
  pub fn holding_these(secret_key: &str, public_key: &str) -> KeyMaster {
    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_str(secret_key).unwrap();
    let public_key = PublicKey::from_str(public_key).unwrap();
    return KeyMaster {
      secp: secp,
      secret_key: secret_key.to_string(),
      public_key: public_key.to_string()
    };
  }

  /* Sign a message */
  pub fn sign(&self, message: String) -> String { 
    let message_ = Message::from_hashed_data::<sha256::Hash>(message.as_bytes());
    return self.secp.sign(&message_, &SecretKey::from_str(&self.secret_key[..]).unwrap()).to_string();
  }

  /* Verify a message */
  pub fn verify(&self, message: String, signature: String) -> bool {
    let message_ = Message::from_hashed_data::<sha256::Hash>(message.as_bytes());
    return self.secp.verify(&message_, &Signature::from_str(&signature[..]).unwrap(),
      &PublicKey::from_str(&self.public_key[..]).unwrap()).is_ok();
  }
}

/* Sign a message */
pub fn sign(keys: &KeyMaster, message: String) -> Signature { 
  let message_ = Message::from_hashed_data::<sha256::Hash>(message.as_bytes());
  return keys.secp.sign(&message_, &SecretKey::from_str(&keys.secret_key[..]).unwrap());
}

/* Verify a message */
pub fn verify(keys: &KeyMaster, message: String, signature: Signature) -> bool {
  let message_ = Message::from_hashed_data::<sha256::Hash>(message.as_bytes());
  return keys.secp.verify(&message_, &signature, &PublicKey::from_str(&keys.public_key[..]).unwrap()).is_ok();
}

/* sha256 */
pub fn hash_string(in_str: &str) -> String {
  let mut hasher = Sha256::new();
  hasher.update(in_str);
  return format!("{:x}", hasher.finalize());
}

