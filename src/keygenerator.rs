use hex_literal::hex;
use sha2::{Sha256, Sha512, Digest};

fn hashTest() {
  // create a Sha256 object
  let mut hasher = Sha256::new();

  // write input message
  hasher.update(b"hello world");

  // read hash digest and consume hasher
  let result = hasher.finalize();
}
