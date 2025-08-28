use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit},
};
use anyhow::{Context, Result};
use base64::{Engine as _, engine::general_purpose};
use rand::{RngCore, thread_rng};

const KEY: &[u8; 32] = b"default-secret-key-32-characters";

pub fn encrypt(plaintext: &str) -> Result<String> {
    // Handle the InvalidLength error explicitly
    let cipher = Aes256Gcm::new_from_slice(KEY)
        .map_err(|e| anyhow::anyhow!("Failed to create cipher: {}", e))?;

    // Generate random nonce - make sure rand is properly imported
    // let nonce_bytes: [u8; 12] = rand::thread_rng().gen();
    let mut nonce_bytes = [0u8; 12];
    thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // let nonce = Nonce::from_slice(&nonce_bytes);

    // Encrypt the plaintext
    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_bytes())
        .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

    // Combine nonce and ciphertext
    let mut result = Vec::new();
    result.extend_from_slice(&nonce_bytes);
    result.extend_from_slice(&ciphertext);

    Ok(format!(
        "{{cipher}}{}",
        general_purpose::STANDARD.encode(result)
    ))
}

pub fn decrypt(encrypted: &str) -> Result<String> {
    // If not encrypted, return as-is
    if !encrypted.starts_with("{cipher}") {
        return Ok(encrypted.to_string());
    }

    // Remove the cipher prefix
    let encrypted_data = encrypted
        .strip_prefix("{cipher}")
        .context("Invalid cipher format")?;

    // Decode base64
    let data = general_purpose::STANDARD
        .decode(encrypted_data)
        .context("Failed to decode base64 data")?;

    // Ensure minimum length
    if data.len() < 12 {
        return Err(anyhow::anyhow!("Invalid encrypted data: too short"));
    }

    // Split nonce and ciphertext
    let (nonce_bytes, ciphertext) = data.split_at(12);

    // Create cipher with explicit error handling
    let cipher = Aes256Gcm::new_from_slice(KEY)
        .map_err(|e| anyhow::anyhow!("Failed to create cipher: {}", e))?;

    let nonce = Nonce::from_slice(nonce_bytes);

    // Decrypt with explicit error handling
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))?;

    // Convert to UTF-8 string
    String::from_utf8(plaintext).context("Decrypted data is not valid UTF-8")
}
