//! Criterion benchmarks for `phenotype-cipher`.
//!
//! The crate exposes two symmetric AEAD ciphers (`AesGcmCipher` for
//! AES-256-GCM and `ChaChaCipher` for ChaCha20-Poly1305), each with a
//! single-shot `encrypt(&[u8]) -> Ciphertext` / `decrypt(&Ciphertext) -> Vec<u8>`
//! API. The benchmarks below cover the requested scenarios:
//!
//!   (a) single-blob encryption/decryption round-trip
//!   (b) 1 MB blob encryption/decryption round-trip
//!   (c) key-size variation — the API is **fixed at 32-byte keys** (AES-256
//!       and ChaCha20 both reject other lengths in `new`), so the only
//!       meaningful key-size axis is "valid vs. rejected". We bench both
//!       algorithms with the supported 32-byte key, plus the rejection
//!       path with an invalid 16-byte key.
//!   (d) streaming — the API has **no native streaming mode** (encrypt
//!       consumes a `&[u8]` and returns a single `Ciphertext`). To still
//!       exercise the "encrypt N chunks" use-case that callers need, we
//!       bench a manual chunked loop that encrypts N×64 KiB chunks and
//!       decrypts them back.
//!
//! Run with:
//!     cargo bench -p phenotype-cipher --bench cipher_bench
//! or filtered:
//!     cargo bench -p phenotype-cipher --bench cipher_bench cipher

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use phenotype_cipher::{AesGcmCipher, ChaChaCipher, Ciphertext};

const SMALL_BLOB: usize = 64;
const KB: usize = 1024;
const CHUNK: usize = 64 * KB;
const NUM_CHUNKS: usize = 16; // 16 * 64 KiB = 1 MiB total, mirrors the 1MB test
const ONE_MIB: usize = 1024 * KB;

// ---------- helpers ---------------------------------------------------------

fn make_payload(len: usize) -> Vec<u8> {
    // Deterministic but non-trivial pattern; using `rand` here would make
    // the bench depend on RNG state and add noise to measurements.
    (0..len).map(|i| (i as u8).wrapping_mul(31)).collect()
}

fn fresh_aes() -> (Vec<u8>, AesGcmCipher) {
    let key = AesGcmCipher::generate_key();
    let cipher = AesGcmCipher::new(&key).expect("32-byte key must construct");
    (key, cipher)
}

fn fresh_chacha() -> (Vec<u8>, ChaChaCipher) {
    let key = ChaChaCipher::generate_key();
    let cipher = ChaChaCipher::new(&key).expect("32-byte key must construct");
    (key, cipher)
}

// ---------- (a) single-blob round-trip --------------------------------------

fn bench_single_blob_roundtrip(c: &mut Criterion) {
    let payload = make_payload(SMALL_BLOB);
    let mut group = c.benchmark_group("cipher_single_blob_roundtrip");
    group.throughput(Throughput::Bytes(SMALL_BLOB as u64));

    group.bench_function(BenchmarkId::new("aes_gcm", SMALL_BLOB), |b| {
        let (_, cipher) = fresh_aes();
        b.iter(|| {
            let ct = cipher.encrypt(black_box(&payload)).unwrap();
            let pt = cipher.decrypt(black_box(&ct)).unwrap();
            assert_eq!(pt.len(), payload.len());
        });
    });

    group.bench_function(BenchmarkId::new("chacha20_poly1305", SMALL_BLOB), |b| {
        let (_, cipher) = fresh_chacha();
        b.iter(|| {
            let ct = cipher.encrypt(black_box(&payload)).unwrap();
            let pt = cipher.decrypt(black_box(&ct)).unwrap();
            assert_eq!(pt.len(), payload.len());
        });
    });

    group.finish();
}

// ---------- (b) 1 MB blob round-trip ----------------------------------------

fn bench_1mb_roundtrip(c: &mut Criterion) {
    let payload = make_payload(ONE_MIB);
    let mut group = c.benchmark_group("cipher_1mb_roundtrip");
    group.throughput(Throughput::Bytes(ONE_MIB as u64));

    // AES-256-GCM encrypt + decrypt on 1 MiB
    group.bench_function("aes_gcm_encrypt", |b| {
        let (_, cipher) = fresh_aes();
        b.iter(|| {
            let ct = cipher.encrypt(black_box(&payload)).unwrap();
            assert!(ct.data.len() >= payload.len());
        });
    });

    group.bench_function("aes_gcm_decrypt", |b| {
        let (_, cipher) = fresh_aes();
        let ct = cipher.encrypt(&payload).unwrap();
        b.iter(|| {
            let pt = cipher.decrypt(black_box(&ct)).unwrap();
            assert_eq!(pt.len(), payload.len());
        });
    });

    group.bench_function("aes_gcm_roundtrip", |b| {
        let (_, cipher) = fresh_aes();
        b.iter(|| {
            let ct = cipher.encrypt(black_box(&payload)).unwrap();
            let pt = cipher.decrypt(black_box(&ct)).unwrap();
            assert_eq!(pt.len(), payload.len());
        });
    });

    // ChaCha20-Poly1305 encrypt + decrypt on 1 MiB
    group.bench_function("chacha20_poly1305_encrypt", |b| {
        let (_, cipher) = fresh_chacha();
        b.iter(|| {
            let ct = cipher.encrypt(black_box(&payload)).unwrap();
            assert!(ct.data.len() >= payload.len());
        });
    });

    group.bench_function("chacha20_poly1305_decrypt", |b| {
        let (_, cipher) = fresh_chacha();
        let ct = cipher.encrypt(&payload).unwrap();
        b.iter(|| {
            let pt = cipher.decrypt(black_box(&ct)).unwrap();
            assert_eq!(pt.len(), payload.len());
        });
    });

    group.bench_function("chacha20_poly1305_roundtrip", |b| {
        let (_, cipher) = fresh_chacha();
        b.iter(|| {
            let ct = cipher.encrypt(black_box(&payload)).unwrap();
            let pt = cipher.decrypt(black_box(&ct)).unwrap();
            assert_eq!(pt.len(), payload.len());
        });
    });

    group.finish();
}

// ---------- (c) key-size variation -----------------------------------------

fn bench_key_sizes(c: &mut Criterion) {
    // The API only supports 32-byte keys (AES-256 / ChaCha20). Any other
    // length is rejected by `new()`. We bench the supported path and the
    // rejection path so we can see validation cost vs. construction cost.
    let mut group = c.benchmark_group("cipher_key_sizes");
    group.bench_function("aes_32byte_key_new", |b| {
        b.iter(|| {
            let key = AesGcmCipher::generate_key();
            let cipher = AesGcmCipher::new(black_box(&key)).unwrap();
            black_box(cipher);
        });
    });

    group.bench_function("aes_16byte_key_new_rejected", |b| {
        let bad_key = vec![0u8; 16];
        b.iter(|| {
            let result = AesGcmCipher::new(black_box(&bad_key));
            assert!(result.is_err());
        });
    });

    group.bench_function("chacha_32byte_key_new", |b| {
        b.iter(|| {
            let key = ChaChaCipher::generate_key();
            let cipher = ChaChaCipher::new(black_box(&key)).unwrap();
            black_box(cipher);
        });
    });

    group.bench_function("chacha_16byte_key_new_rejected", |b| {
        let bad_key = vec![0u8; 16];
        b.iter(|| {
            let result = ChaChaCipher::new(black_box(&bad_key));
            assert!(result.is_err());
        });
    });

    group.finish();
}

// ---------- (d) chunked / "streaming" simulation ----------------------------

fn bench_chunked_roundtrip(c: &mut Criterion) {
    // phenotype-cipher's `encrypt` is single-shot (no streaming). We
    // simulate streaming by encrypting N independent chunks of 64 KiB
    // each (totalling 1 MiB), which is the pattern callers must use today
    // if they want a "chunked" pipeline. The result is a Vec<Ciphertext>
    // of N items.
    let chunks: Vec<Vec<u8>> = (0..NUM_CHUNKS).map(|_| make_payload(CHUNK)).collect();
    let total = CHUNK * NUM_CHUNKS;
    let mut group = c.benchmark_group("cipher_chunked_roundtrip");
    group.throughput(Throughput::Bytes(total as u64));

    group.bench_function("aes_gcm_16x64kb_encrypt", |b| {
        let (_, cipher) = fresh_aes();
        b.iter(|| {
            let mut out: Vec<Ciphertext> = Vec::with_capacity(NUM_CHUNKS);
            for chunk in &chunks {
                let ct = cipher.encrypt(black_box(chunk.as_slice())).unwrap();
                out.push(ct);
            }
            assert_eq!(out.len(), NUM_CHUNKS);
        });
    });

    group.bench_function("aes_gcm_16x64kb_decrypt", |b| {
        let (_, cipher) = fresh_aes();
        let encrypted: Vec<Ciphertext> = chunks
            .iter()
            .map(|c| cipher.encrypt(c.as_slice()).unwrap())
            .collect();
        b.iter(|| {
            let mut out_len = 0usize;
            for ct in &encrypted {
                let pt = cipher.decrypt(black_box(ct)).unwrap();
                out_len += pt.len();
            }
            assert_eq!(out_len, total);
        });
    });

    group.bench_function("chacha20_poly1305_16x64kb_encrypt", |b| {
        let (_, cipher) = fresh_chacha();
        b.iter(|| {
            let mut out: Vec<Ciphertext> = Vec::with_capacity(NUM_CHUNKS);
            for chunk in &chunks {
                let ct = cipher.encrypt(black_box(chunk.as_slice())).unwrap();
                out.push(ct);
            }
            assert_eq!(out.len(), NUM_CHUNKS);
        });
    });

    group.bench_function("chacha20_poly1305_16x64kb_decrypt", |b| {
        let (_, cipher) = fresh_chacha();
        let encrypted: Vec<Ciphertext> = chunks
            .iter()
            .map(|c| cipher.encrypt(c.as_slice()).unwrap())
            .collect();
        b.iter(|| {
            let mut out_len = 0usize;
            for ct in &encrypted {
                let pt = cipher.decrypt(black_box(ct)).unwrap();
                out_len += pt.len();
            }
            assert_eq!(out_len, total);
        });
    });

    group.finish();
}

// ---------- (e) algorithm x payload-size round-trip -------------------------

fn bench_algorithm_roundtrip(c: &mut Criterion) {
    // Cross-product benchmark: every algorithm exposed by the crate x a small
    // grid of payload sizes (1 KB, 10 KB, 100 KB). Each bench_function does a
    // full encrypt+decrypt round-trip, so the measurement covers the cipher's
    // cost on a representative single-shot payload (the only mode the API
    // exposes today).
    //
    // NOTE: XChaCha20-Poly1305 is intentionally NOT included. The
    // `phenotype-cipher` crate does not currently expose an XChaCha20-Poly1305
    // type — the public API surface is limited to `AesGcmCipher`
    // (AES-256-GCM) and `ChaChaCipher` (ChaCha20-Poly1305). Adding a third
    // algorithm here would require extending the crate's public API, which
    // is out of scope for this bench-only change.
    let sizes: &[(usize, &str)] = &[
        (KB, "1kb"),
        (10 * KB, "10kb"),
        (100 * KB, "100kb"),
    ];

    for (size, label) in sizes {
        let payload = make_payload(*size);
        let mut group = c.benchmark_group("cipher_algorithm_roundtrip");
        group.throughput(Throughput::Bytes(*size as u64));

        group.bench_function(BenchmarkId::new("aes_gcm", label), |b| {
            let (_, cipher) = fresh_aes();
            b.iter(|| {
                let ct = cipher.encrypt(black_box(&payload)).unwrap();
                let pt = cipher.decrypt(black_box(&ct)).unwrap();
                assert_eq!(pt.len(), payload.len());
            });
        });

        group.bench_function(BenchmarkId::new("chacha20_poly1305", label), |b| {
            let (_, cipher) = fresh_chacha();
            b.iter(|| {
                let ct = cipher.encrypt(black_box(&payload)).unwrap();
                let pt = cipher.decrypt(black_box(&ct)).unwrap();
                assert_eq!(pt.len(), payload.len());
            });
        });

        group.finish();
    }
}

// ---------- entry points ----------------------------------------------------

criterion_group!(
    benches,
    bench_single_blob_roundtrip,
    bench_1mb_roundtrip,
    bench_key_sizes,
    bench_chunked_roundtrip,
    bench_algorithm_roundtrip
);
criterion_main!(benches);
