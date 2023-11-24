// region:    --- Modules
mod error;
mod hmac_hasher;

use crate::config::config;

pub use self::error::{Error, Result};
use uuid::Uuid;
// endRegion: --- Modules

// region:    --- types
pub struct ContentToHash {
    pub content: String, // Clear content.
    pub salt: Uuid,      // Clear Salt
}
// endRegion: --- types

// region:    --- public function
// --- Hash the password with the default scheme
pub fn hash_pwd(to_hash: &ContentToHash) -> Result<String> {
    let key = &config().PWD_KEY;
    let hashed = ""; // TODO
    Ok(format!("#01#{hashed}"))
}

// --- Validate if an ContentToHash matches
pub fn validate_pwd(enc_content: &ContentToHash, pwd_ref: &str) -> Result<()> {
    let pwd = hash_pwd(enc_content)?;

    if pwd == pwd_ref {
        Ok(())
    } else {
        Err(Error::NotMatching)
    }
}
// endRegion: --- public function
