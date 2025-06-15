use sha2::digest::Update;
use sha2::{Digest, Sha256};

/// Construct a key for the given payload
pub fn construct_key(payload: &str) -> String {
    let mut hasher = Sha256::new();
    Update::update(&mut hasher, payload.as_bytes());

    let hash = hasher.finalize();
    let hash_str = format!("{:x}", hash);

    hash_str
}

/// Verify the payload against the given hash
pub fn verify_payload(hash: &str, payload: &str) -> bool {
    let hash_str = construct_key(payload);
    hash_str == hash
}
