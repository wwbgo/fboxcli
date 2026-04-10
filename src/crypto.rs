use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use aes_gcm::aead::Aead;
use anyhow::{Context, Result};
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use sha2::{Sha256, Digest};

const ENC_PREFIX: &str = "ENC:";
const NONCE_LEN: usize = 12;
/// Fixed seed for key derivation — keeps encrypted configs portable across machines.
const KEY_SEED: &[u8] = b"fboxcli-aes256gcm-fixed-key-seed-2026";

/// Derive a 256-bit encryption key from the fixed seed.
fn derive_key() -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(KEY_SEED);
    hasher.finalize().into()
}

/// Encrypt a plaintext string. Returns `ENC:<base64(nonce + ciphertext)>`.
pub fn encrypt(plaintext: &str) -> Result<String> {
    if plaintext.is_empty() {
        return Ok(String::new());
    }

    let key = derive_key();
    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|e| anyhow::anyhow!("Failed to create cipher: {}", e))?;

    let mut nonce_bytes = [0u8; NONCE_LEN];
    rand::fill(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, plaintext.as_bytes())
        .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

    let mut combined = Vec::with_capacity(NONCE_LEN + ciphertext.len());
    combined.extend_from_slice(&nonce_bytes);
    combined.extend_from_slice(&ciphertext);

    Ok(format!("{}{}", ENC_PREFIX, BASE64.encode(&combined)))
}

/// Decrypt a string. If it starts with `ENC:`, decrypt it; otherwise return as-is
/// (backward compatible with unencrypted configs).
pub fn decrypt(value: &str) -> Result<String> {
    if value.is_empty() || !value.starts_with(ENC_PREFIX) {
        return Ok(value.to_string());
    }

    let encoded = &value[ENC_PREFIX.len()..];
    let combined = BASE64.decode(encoded)
        .context("Failed to decode base64")?;

    if combined.len() < NONCE_LEN {
        anyhow::bail!("Invalid encrypted data: too short");
    }

    let (nonce_bytes, ciphertext) = combined.split_at(NONCE_LEN);
    let key = derive_key();
    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|e| anyhow::anyhow!("Failed to create cipher: {}", e))?;
    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = cipher.decrypt(nonce, ciphertext)
        .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))?;

    String::from_utf8(plaintext).context("Decrypted data is not valid UTF-8")
}
