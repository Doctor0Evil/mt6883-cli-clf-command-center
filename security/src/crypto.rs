use base64::{engine::general_purpose, Engine as _};
use ring::aead::{self, Aad, LessSafeKey, UnboundKey};
use ring::rand::SystemRandom;

pub fn encrypt_aes256(key_bytes: &[u8; 32], plaintext: &[u8]) -> Vec<u8> {
    let rng = SystemRandom::new();
    let mut nonce_bytes = [0u8; 12];
    ring::rand::generate(&rng, &mut nonce_bytes).unwrap();

    let unbound_key =
        UnboundKey::new(&aead::AES_256_GCM, key_bytes).expect("key");
    let key = LessSafeKey::new(unbound_key);
    let nonce = aead::Nonce::assume_unique_for_key(nonce_bytes);
    let mut in_out = plaintext.to_vec();
    key.seal_in_place_append_tag(nonce, Aad::empty(), &mut in_out)
        .unwrap();

    let mut result = nonce_bytes.to_vec();
    result.extend_from_slice(&in_out);
    result
}

pub fn encrypt_aes256_base64(key_bytes: &[u8; 32], plaintext: &[u8]) -> String {
    let data = encrypt_aes256(key_bytes, plaintext);
    general_purpose::STANDARD.encode(data)
}
