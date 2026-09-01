//! Pointer to canonical `phenotype-router` v0.2.0 in `KooshaPari/phenotype-router`.
//!
//! HexaKit previously maintained its own `phenotype-router` v0.1.0 here. After
//! the 2026-09-01 polyrepo audit, the standalone `KooshaPari/phenotype-router` is
//! the canonical home (v0.2.0, OTel-aware, with ADR-049/050/051).
//!
//! This crate is a **thin pointer** to keep HexaKit's workspace member list
//! stable while dependent crates migrate. New code should depend directly on
//! the standalone repo:
//!
//! ```toml
//! [dependencies]
//! phenotype-router = { git = "https://github.com/KooshaPari/phenotype-router", tag = "v0.2.0" }
//! ```
//!
//! See `crates/POINTER-README.md` for full migration notes.

#![allow(unused_imports)]
pub use phenotype_router_standalone::*;
