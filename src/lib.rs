use sha2::Sha256;
use digest::Digest;
use std::fs::File;
use std::io::Read;

pub fn calculate_hash(mut file: &File) -> String {
  let mut hasher = Sha256::new();
  let mut buffer: [u8; 1024] = [0; 1024]; // Use a buffer for reading
  loop {
    match file.read(&mut buffer) {
      Ok(bytes_read) => {
        if bytes_read == 0 {
          break;
        }
        hasher.update(&buffer[..bytes_read]);
      }
      Err(err) => {
        eprintln!("Error reading file: {}", err);
        break;
      }
    }
  }

  let hash_result = hasher.finalize();
  let hash_hex_string: String = hash_result
    .iter()
    .map(|byte| format!("{:02x}", byte))
    .collect();

  println!("SHA-256 Hash: {}", hash_hex_string);

  hash_hex_string
}
