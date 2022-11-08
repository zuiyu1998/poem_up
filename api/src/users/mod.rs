use sha2::{Digest, Sha256};

pub mod api;
pub mod models;
pub mod routes;

pub use routes::*;

pub fn spawn_password(raw: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(b"poem_up");
    hasher.update(raw.as_bytes());

    let result = hasher.finalize().to_vec();

    result
}

pub fn uid() -> String {
    xid::new().to_string()
}
