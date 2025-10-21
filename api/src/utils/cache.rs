use sha2::{Sha256, Digest};
use hex;
use serde::Serialize;

pub fn build_cache_key<T: Serialize>(prefix: &str, obj: &T) -> String {
    let mut hasher = Sha256::new();
    let payload = serde_json::to_string(obj).unwrap_or_default();
    hasher.update(payload.as_bytes());
    format!("{}:{}", prefix, hex::encode(hasher.finalize()))
}