//! HexaKit `templates/hexagon/rust/` lib — cargo-generate target root.
//!
//! Absorbed 2026-06-20 from:
//!   - `KooshaPari/pheno-cargo-template` (root crate + `template/` partials)
//!   - `KooshaPari/Apisync` (templates/hexagonal/ overlay, commit d981353)
//!
//! Public surface: a single `crate_name()` accessor that lets a generated
//! crate verify it came from this template (mirrors `pheno-scaffold-kit`'s
//! `crate_name` check from the absorbed pheno-cargo-template).

/// Returns the canonical name of this cargo-generate target crate.
///
/// Consumed by `pheno-scaffold-kit` (and any consumer running
/// `cargo generate --git https://github.com/KooshaPari/HexaKit --subfolder templates/hexagon/rust`)
/// to verify a generated crate was scaffolded from this template.
pub fn crate_name() -> &'static str {
    "hexagon-rust-template"
}

/// Returns the canonical git source for this template.
///
/// Mirrors the `repository = ...` field in `Cargo.toml` so a consumer
/// can record provenance without parsing the manifest.
pub fn template_source() -> &'static str {
    "https://github.com/KooshaPari/HexaKit/tree/main/templates/hexagon/rust"
}

/// Returns the absorption provenance for this template.
pub fn absorbed_from() -> &'static str {
    "pheno-cargo-template (2026-06-20) + Apisync/templates/hexagonal (d981353, 2026-06-19)"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crate_name_matches_package() {
        assert_eq!(crate_name(), "hexagon-rust-template");
    }

    #[test]
    fn template_source_points_to_hexakit() {
        assert!(template_source().contains("HexaKit/templates/hexagon/rust"));
    }

    #[test]
    fn absorbed_from_lists_both_sources() {
        let s = absorbed_from();
        assert!(s.contains("pheno-cargo-template"));
        assert!(s.contains("Apisync"));
    }
}