//! Integration tests for `phenotype-cipher`.
//!
//! These tests live in `tests/` (not `#[cfg(test)] mod tests` inside `src/`) so
//! they exercise the **public** crate API only. The intent is to add a second
//! layer of coverage on top of the existing unit tests and to give downstream
//! consumers of the crate something to lean on when reasoning about the
//! behavior of the cipher primitives.
//!
//! Coverage areas:
//! 1. Encryption round-trip (AES-GCM and ChaCha20-Poly1305).
//! 2. Determinism of the key-derived primitives (HMAC-SHA256, and the
//!    invariant that the same `Ciphertext` always decrypts to the same
//!    plaintext under the same key). Note: AEAD ciphers in this crate use a
//!    fresh random nonce per `encrypt`, so "same key → same ciphertext" is
//!    not a property of `encrypt` itself; determinism is verified on the
//!    key-derived MAC and on the decrypt direction instead.
//! 3. Different keys → different ciphertexts for the same plaintext.
//! 4. Tamper detection (flipping a byte in ciphertext or nonce must cause
//!    `decrypt` to return an error).
//! 5. Key serialization round-trip for `PublicKey` and `SecretKey`, plus a
//!    sign/verify round-trip through the raw byte representation.
//!
//! These tests are intentionally API-level only — they only call items
//! re-exported at the crate root.

use phenotype_cipher::{
    AesGcmCipher, ChaChaCipher, Ciphertext, Ed25519Signer, PublicKey, SecretKey, Sha256Hasher,
};
// The crate's root re-exports `Signature` (alias for `SignatureBytes`); the
// HMAC helpers are only reachable through the `core` module. Both `core`
// itself and `hashing` are declared `pub` in `lib.rs`, so these paths are
// part of the public API surface.
use phenotype_cipher::core::hashing::{hmac_sha256, verify_hmac_sha256};
use phenotype_cipher::core::signatures::SignatureBytes;

// ---------------------------------------------------------------------------
// 1. Encryption round-trip
// ---------------------------------------------------------------------------

#[test]
fn aes_gcm_encrypt_decrypt_round_trip_matches_plaintext() {
    let key = AesGcmCipher::generate_key();
    let cipher = AesGcmCipher::new(&key).expect("32-byte key must construct");
    let plaintext = b"the quick brown fox jumps over the lazy dog";

    let ciphertext: Ciphertext = cipher.encrypt(plaintext).expect("encrypt must succeed");
    let decrypted = cipher.decrypt(&ciphertext).expect("decrypt must succeed");

    assert_eq!(
        decrypted.as_slice(),
        plaintext.as_slice(),
        "AES-GCM round-trip must yield the original plaintext"
    );
}

#[test]
fn chacha20_encrypt_decrypt_round_trip_matches_plaintext() {
    let key = ChaChaCipher::generate_key();
    let cipher = ChaChaCipher::new(&key).expect("32-byte key must construct");
    let plaintext = b"hello chacha20-poly1305";

    let ciphertext = cipher.encrypt(plaintext).expect("encrypt must succeed");
    let decrypted = cipher.decrypt(&ciphertext).expect("decrypt must succeed");

    assert_eq!(decrypted.as_slice(), plaintext.as_slice());
}

#[test]
fn aes_gcm_empty_plaintext_round_trips() {
    let cipher = AesGcmCipher::new(&AesGcmCipher::generate_key()).unwrap();
    let ct = cipher.encrypt(b"").expect("encrypt empty must succeed");
    let pt = cipher.decrypt(&ct).expect("decrypt empty must succeed");
    assert!(pt.is_empty(), "round-trip of empty plaintext must be empty");
}

#[test]
fn chacha_empty_plaintext_round_trips() {
    let cipher = ChaChaCipher::new(&ChaChaCipher::generate_key()).unwrap();
    let ct = cipher.encrypt(b"").expect("encrypt empty must succeed");
    let pt = cipher.decrypt(&ct).expect("decrypt empty must succeed");
    assert!(pt.is_empty(), "round-trip of empty plaintext must be empty");
}

#[test]
fn large_plaintext_round_trips() {
    let key = AesGcmCipher::generate_key();
    let cipher = AesGcmCipher::new(&key).unwrap();
    let plaintext = vec![0xABu8; 64 * 1024]; // 64 KiB

    let ct = cipher.encrypt(&plaintext).expect("encrypt 64KiB must succeed");
    let pt = cipher.decrypt(&ct).expect("decrypt 64KiB must succeed");
    assert_eq!(pt, plaintext, "64KiB round-trip must match exactly");
}

// ---------------------------------------------------------------------------
// 2. Determinism of key-derived primitives
// ---------------------------------------------------------------------------
//
// AEAD `encrypt` is intentionally non-deterministic: each call draws a fresh
// random nonce. We therefore verify determinism on the *key-derived* primitive
// that the crate exposes (HMAC-SHA256) and on the decryption direction of the
// AEAD ciphers (the same `Ciphertext` under the same key must always decrypt
// to the same plaintext).

#[test]
fn hmac_sha256_is_deterministic_under_same_key_and_data() {
    let key = b"deterministic-key";
    let data = b"some message";

    let mac1 = hmac_sha256(key, data);
    let mac2 = hmac_sha256(key, data);

    assert_eq!(
        mac1, mac2,
        "HMAC-SHA256 must be deterministic for the same key+data"
    );
    assert_eq!(mac1.len(), 32, "HMAC-SHA256 output is always 32 bytes");
}

#[test]
fn sha256_hasher_is_deterministic() {
    // SHA-256 of the same input must always produce the same digest. This
    // also serves as a sanity check on the hash facade.
    let h1 = Sha256Hasher::hash(b"deterministic-input");
    let h2 = Sha256Hasher::hash(b"deterministic-input");
    assert_eq!(h1, h2);
    assert_eq!(h1.len(), 32);
}

#[test]
fn decrypting_same_ciphertext_is_deterministic() {
    // Decrypt is a pure function of (key, ciphertext) and must therefore
    // always return the same plaintext when invoked repeatedly.
    let key = AesGcmCipher::generate_key();
    let cipher = AesGcmCipher::new(&key).unwrap();
    let ct = cipher.encrypt(b"repeat me").unwrap();

    let pt1 = cipher.decrypt(&ct).unwrap();
    let pt2 = cipher.decrypt(&ct).unwrap();
    let pt3 = cipher.decrypt(&ct).unwrap();

    assert_eq!(pt1, pt2);
    assert_eq!(pt2, pt3);
    assert_eq!(pt1, b"repeat me");
}

// ---------------------------------------------------------------------------
// 3. Different keys → different ciphertexts
// ---------------------------------------------------------------------------

#[test]
fn aes_gcm_different_keys_produce_different_ciphertexts() {
    let key_a = AesGcmCipher::generate_key();
    let key_b = AesGcmCipher::generate_key();
    assert_ne!(key_a, key_b, "two generated keys must differ");

    let cipher_a = AesGcmCipher::new(&key_a).unwrap();
    let cipher_b = AesGcmCipher::new(&key_b).unwrap();

    let plaintext = b"identical plaintext under different keys";

    // We can't make the nonce deterministic, so to be robust to nonce
    // randomness we compare the *decryption failure* behavior: ciphertext
    // produced under key A must not decrypt under key B.
    let ct_under_a = cipher_a.encrypt(plaintext).unwrap();
    let result = cipher_b.decrypt(&ct_under_a);

    assert!(
        result.is_err(),
        "ciphertext produced under key A must not decrypt under key B"
    );
}

#[test]
fn chacha_different_keys_produce_undecryptable_ciphertexts() {
    let key_a = ChaChaCipher::generate_key();
    let key_b = ChaChaCipher::generate_key();
    assert_ne!(key_a, key_b);

    let cipher_a = ChaChaCipher::new(&key_a).unwrap();
    let cipher_b = ChaChaCipher::new(&key_b).unwrap();

    let ct = cipher_a.encrypt(b"same plaintext").unwrap();
    assert!(
        cipher_b.decrypt(&ct).is_err(),
        "ciphertext from key A must not decrypt under key B"
    );
}

#[test]
fn hmac_different_keys_produce_different_macs() {
    let data = b"the same message";
    let mac_a = hmac_sha256(b"key-a", data);
    let mac_b = hmac_sha256(b"key-b", data);
    assert_ne!(
        mac_a, mac_b,
        "HMAC must change when the key changes"
    );
}

// ---------------------------------------------------------------------------
// 4. Tamper detection
// ---------------------------------------------------------------------------

#[test]
fn aes_gcm_tampered_ciphertext_fails_to_decrypt() {
    let key = AesGcmCipher::generate_key();
    let cipher = AesGcmCipher::new(&key).unwrap();
    let mut ct = cipher.encrypt(b"authenticated payload").unwrap();

    // Flip a single bit in the ciphertext body. AES-GCM's authentication tag
    // must catch this.
    assert!(!ct.data.is_empty(), "ciphertext body must be non-empty");
    ct.data[0] ^= 0x01;

    let result = cipher.decrypt(&ct);
    assert!(
        result.is_err(),
        "decrypting a tampered AES-GCM ciphertext must fail"
    );
}

#[test]
fn chacha_tampered_ciphertext_fails_to_decrypt() {
    let key = ChaChaCipher::generate_key();
    let cipher = ChaChaCipher::new(&key).unwrap();
    let mut ct = cipher.encrypt(b"authenticated payload").unwrap();

    assert!(!ct.data.is_empty());
    ct.data[0] ^= 0x80;

    assert!(
        cipher.decrypt(&ct).is_err(),
        "decrypting a tampered ChaCha20-Poly1305 ciphertext must fail"
    );
}

#[test]
fn aes_gcm_tampered_nonce_fails_to_decrypt() {
    let key = AesGcmCipher::generate_key();
    let cipher = AesGcmCipher::new(&key).unwrap();
    let mut ct = cipher.encrypt(b"nonce sensitivity").unwrap();

    // Mutating the nonce with a different key must cause AES-GCM to fail
    // authentication (and not silently produce a wrong plaintext).
    ct.nonce[0] ^= 0xFF;
    assert!(
        cipher.decrypt(&ct).is_err(),
        "AES-GCM must reject ciphertexts with mutated nonces"
    );
}

#[test]
fn hmac_rejects_modified_data() {
    let key = b"integrity-key";
    let data = b"original message";
    let mac = hmac_sha256(key, data);

    assert!(
        verify_hmac_sha256(key, data, &mac),
        "HMAC must verify under the original data"
    );
    assert!(
        !verify_hmac_sha256(key, b"modified message", &mac),
        "HMAC must not verify under modified data"
    );
}

// ---------------------------------------------------------------------------
// 5. Key serialization round-trip
// ---------------------------------------------------------------------------

#[test]
fn public_key_serialization_round_trip() {
    let (pk, _sk) = Ed25519Signer::generate_keypair();
    let bytes = pk.as_bytes().to_vec();

    assert_eq!(bytes.len(), 32, "Ed25519 public key is 32 bytes");

    // Round-trip: bytes → PublicKey → bytes
    let pk2 = PublicKey::from(bytes.clone());
    assert_eq!(pk2.as_bytes(), bytes.as_slice());
    assert_eq!(pk2, pk, "PublicKey equality must hold after round-trip");

    // `to_bytes` consumes self; verify it agrees with `as_bytes`.
    let pk3 = PublicKey::from(bytes);
    assert_eq!(pk3.to_bytes(), pk.as_bytes());
}

#[test]
fn secret_key_serialization_round_trip() {
    let (_pk, sk) = Ed25519Signer::generate_keypair();
    let bytes_via_as = sk.as_bytes().to_vec();
    let bytes_via_to = sk.to_bytes();

    assert_eq!(bytes_via_as, bytes_via_to, "as_bytes and to_bytes must agree");
    assert_eq!(bytes_via_to.len(), 32, "Ed25519 secret key is 32 bytes");

    // Round-trip: bytes → SecretKey → bytes
    let sk2 = SecretKey::from(bytes_via_to.clone());
    assert_eq!(sk2.as_bytes(), bytes_via_to);
    assert_eq!(sk2.to_bytes(), bytes_via_to);
}

#[test]
fn sign_and_verify_round_trip_through_raw_bytes() {
    // Produce a keypair, sign a message, then re-construct both keys from
    // raw bytes and verify that the signature still validates. This is the
    // "key serialization round-trip" end-to-end check.
    let (pk, sk) = Ed25519Signer::generate_keypair();
    let message = b"payload to sign";

    let sig = Ed25519Signer::sign(message, &sk)
        .expect("signing with a valid key must succeed");

    // Reconstruct keys from bytes.
    let pk_bytes = pk.as_bytes().to_vec();
    let sk_bytes = sk.as_bytes().to_vec();
    let pk_restored = PublicKey::from(pk_bytes);
    let sk_restored = SecretKey::from(sk_bytes);

    // The signature should still verify under the restored public key, and
    // a freshly-produced signature using the restored secret key must equal
    // the original (Ed25519 is deterministic).
    let verified = Ed25519Signer::verify(message, &sig, &pk_restored)
        .expect("verify must not error for well-formed inputs");
    assert!(verified, "signature must verify under the restored public key");

    let sig_again =
        Ed25519Signer::sign(message, &sk_restored).expect("sign with restored key must succeed");
    assert_eq!(
        sig_again.as_bytes(),
        sig.as_bytes(),
        "Ed25519 signatures are deterministic for the same key+message"
    );
}

#[test]
fn signature_bytes_serialization_round_trip() {
    let (_pk, sk) = Ed25519Signer::generate_keypair();
    let sig: SignatureBytes =
        Ed25519Signer::sign(b"hello", &sk).expect("signing must succeed");

    let bytes = sig.as_bytes().to_vec();
    assert_eq!(bytes.len(), 64, "Ed25519 signature is 64 bytes");

    let sig2 = SignatureBytes::from(bytes.clone());
    assert_eq!(sig2.as_bytes(), bytes.as_slice());
    assert_eq!(sig2, sig, "SignatureBytes equality must hold after round-trip");

    // `to_bytes` consumes self; it must agree with `as_bytes`.
    let sig3 = SignatureBytes::from(bytes);
    assert_eq!(sig3.to_bytes(), sig.as_bytes());
}
