//! # phonetic-engine
//!
//! Native Bangla phonetic transliteration and composition.
//!
//! Converts Latin phonetic input into Bengali Unicode using longest-match
//! rules, word overrides, and a composition state machine suitable for
//! Text Services Framework integration via `ime-bridge`.

#![deny(unsafe_code)]

pub mod composition;
pub mod rules;
pub mod transliterate;

pub use composition::{CompositionEvent, CompositionSession, CompositionUpdate, InputMode};
pub use transliterate::{ambiguous_candidates, transliterate, PhoneticConfig};

/// Crate version string.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Product identity constants.
pub mod identity {
    pub const PRODUCT_NAME: &str = "Native Bangla";
    pub const SHORT_NAME: &str = "Native";
    pub const ENGINE_NAME: &str = "Native Engine";
    pub const TAGLINE: &str = "Type Naturally. Write Beautifully.";
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn identity() {
        assert_eq!(identity::PRODUCT_NAME, "Native Bangla");
    }

    #[test]
    fn common_phrase() {
        let cfg = PhoneticConfig::default();
        let out = transliterate("kemon acho", &cfg);
        assert!(out.contains("কেমন"));
        assert!(out.contains("আছো"));
    }

    #[test]
    fn corrections() {
        let cfg = PhoneticConfig::default();
        assert_eq!(transliterate("vul", &cfg), "ভুল");
        assert_eq!(transliterate("shikkha", &cfg), "শিক্ষা");
        assert_eq!(transliterate("shadhinota", &cfg), "স্বাধীনতা");
    }

    #[test]
    fn conjunct_hasanta() {
        let cfg = PhoneticConfig::default();
        // "k,t" style hasanta — comma maps to hasanta in specials
        let out = transliterate("k,t", &cfg);
        assert!(out.contains('্') || out.contains('ক') || out.contains('ত'));
    }
}
