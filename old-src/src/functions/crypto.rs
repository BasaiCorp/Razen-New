use crate::value::Value;
use sha2::{Sha256, Digest};
use base64::{engine::general_purpose, Engine as _};
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce
};
use aes_gcm::AeadCore;
use hkdf::Hkdf;
use sha2::Sha256 as HkdfSha256;

/// Hashes a string using SHA-256
/// Example: hash("abc") => "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
pub fn hash(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Crypto.hash requires exactly 1 argument: string".to_string());
    }
    
    let input = args[0].as_string()?;
    
    // Create a SHA-256 hasher
    let mut hasher = Sha256::new();
    
    // Update the hasher with the input
    hasher.update(input.as_bytes());
    
    // Get the hash result
    let result = hasher.finalize();
    
    // Convert to hex string
    let hex_string = format!("{:x}", result);
    
    Ok(Value::String(hex_string))
}

/// Encrypts a string with a key using AES-256-GCM
/// Example: encrypt("message", "key") => "encrypted_data"
pub fn encrypt(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("Crypto.encrypt requires exactly 2 arguments: string, key".to_string());
    }
    
    let plaintext = args[0].as_string()?;
    let key_str = args[1].as_string()?;
    
    // Derive a proper length key using HKDF
    let mut derived_key = [0u8; 32]; // AES-256 needs a 32-byte key
    let hkdf = Hkdf::<HkdfSha256>::new(None, key_str.as_bytes());
    hkdf.expand(b"aes-256-gcm", &mut derived_key)
        .map_err(|_| "Key derivation failed".to_string())?;
    
    // Create cipher instance
    let cipher = Aes256Gcm::new_from_slice(&derived_key)
        .map_err(|_| "Failed to create cipher".to_string())?;
    
    // Generate a random 96-bit nonce
    let nonce_bytes = Aes256Gcm::generate_nonce(&mut OsRng);
    let nonce = Nonce::from_slice(nonce_bytes.as_slice());
    
    // Encrypt the plaintext
    let ciphertext = cipher.encrypt(nonce, plaintext.as_bytes())
        .map_err(|_| "Encryption failed".to_string())?;
    
    // Combine nonce and ciphertext and encode as base64
    let mut combined = nonce_bytes.to_vec();
    combined.extend_from_slice(&ciphertext);
    let encoded = general_purpose::STANDARD.encode(&combined);
    
    Ok(Value::String(encoded))
}

/// Decrypts a string with a key using AES-256-GCM
/// Example: decrypt("encrypted_data", "key") => "message"
pub fn decrypt(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("Crypto.decrypt requires exactly 2 arguments: encrypted_string, key".to_string());
    }
    
    let encrypted = args[0].as_string()?;
    let key_str = args[1].as_string()?;
    
    // Decode the base64 input
    let combined = general_purpose::STANDARD.decode(encrypted.as_bytes())
        .map_err(|_| "Invalid base64 encoding".to_string())?;
    
    // Extract nonce (first 12 bytes) and ciphertext
    if combined.len() <= 12 {
        return Err("Invalid encrypted data".to_string());
    }
    let nonce_bytes = &combined[..12];
    let ciphertext = &combined[12..];
    
    // Derive a proper length key using HKDF
    let mut derived_key = [0u8; 32]; // AES-256 needs a 32-byte key
    let hkdf = Hkdf::<HkdfSha256>::new(None, key_str.as_bytes());
    hkdf.expand(b"aes-256-gcm", &mut derived_key)
        .map_err(|_| "Key derivation failed".to_string())?;
    
    // Create cipher instance
    let cipher = Aes256Gcm::new_from_slice(&derived_key)
        .map_err(|_| "Failed to create cipher".to_string())?;
    
    // Create nonce
    let nonce = Nonce::from_slice(nonce_bytes);
    
    // Decrypt the ciphertext
    let plaintext = cipher.decrypt(nonce, ciphertext)
        .map_err(|_| "Decryption failed. The key may be incorrect.".to_string())?;
    
    // Convert plaintext bytes to string
    let plaintext_str = String::from_utf8(plaintext)
        .map_err(|_| "Decrypted data is not valid UTF-8".to_string())?;
    
    Ok(Value::String(plaintext_str))
}
