//! {{crate_name}} — cargo-generate target output library.
//!
//! Scaffolded 2026-06-20 from `KooshaPari/HexaKit/templates/hexagon/rust/`
//! (absorbed `KooshaPari/pheno-cargo-template` + `KooshaPari/Apisync`).
//!
//! Edit this file freely. The absorb anchor is `crate_name()` below —
//! keep that symbol or update any consumer that verifies provenance.

/// Canonical crate name as it appears in `[package].name`.
pub fn crate_name() -> &'static str {
    "{{crate_name}}"
}

/// Returns the template source that scaffolded this crate.
pub fn template_source() -> &'static str {
    "https://github.com/KooshaPari/HexaKit/tree/main/templates/hexagon/rust"
}

/// Returns the absorb provenance for the HexaKit `hexagon/rust` target.
pub fn absorbed_from() -> &'static str {
    "pheno-cargo-template (2026-06-20) + Apisync/templates/hexagonal (d981353, 2026-06-19)"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crate_name_is_non_empty() {
        assert!(!crate_name().is_empty());
    }

    #[test]
    fn template_source_points_to_hexakit() {
        assert!(template_source().contains("HexaKit/templates/hexagon/rust"));
    }
}