

use chacha20poly1305::{
    ChaCha20Poly1305,
    Key,
    Nonce,
    aead::{Aead, KeyInit},
};

fn Encryptpacket(packet:&[u8])-> Vec<u8>{
    let key = Key::from_slice(&[0u8; 32]);

    let cipher = ChaCha20Poly1305::new(key);

    let nonce = Nonce::from_slice(&[1u8; 12]);

    let ciphertext = cipher
    .encrypt(nonce, packet.as_ref())
    .expect("encryption failed");

    return ciphertext;
}


fn Decryptpacket(packet:&[u8])-> Vec<u8>{
    let key = Key::from_slice(&[0u8; 32]);

    let cipher = ChaCha20Poly1305::new(key);

    let nonce = Nonce::from_slice(&[1u8; 12]);

    let plaintext = cipher
    .decrypt(nonce, packet.as_ref())
    .expect("decryption failed");

    return plaintext;
}