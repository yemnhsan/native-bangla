//! Longest-match phonetic transliteration for Native Bangla.

use crate::rules::{
    is_bengali_consonant, is_bengali_vowel, to_kar, ORDERED_RULES, WORD_OVERRIDES,
};
use unicode_normalization::UnicodeNormalization;

const HASANTA: &str = "্";

/// Configuration for phonetic conversion.
#[derive(Debug, Clone)]
pub struct PhoneticConfig {
    /// Prefer word-level overrides for common spellings.
    pub use_word_overrides: bool,
    /// Convert ASCII digits to Bengali digits.
    pub bengali_digits: bool,
    /// Convert ASCII period to danda (।).
    pub convert_danda: bool,
}

impl Default for PhoneticConfig {
    fn default() -> Self {
        Self {
            use_word_overrides: true,
            bengali_digits: false,
            convert_danda: true,
        }
    }
}

/// Transliterate a phonetic (Latin) string into Bengali Unicode.
pub fn transliterate(input: &str, config: &PhoneticConfig) -> String {
    if input.is_empty() {
        return String::new();
    }

    // Preserve English-looking tokens when they look like URLs / emails / paths.
    if should_preserve_latin(input) {
        return input.to_string();
    }

    if config.use_word_overrides {
        let lower = input.to_ascii_lowercase();
        if let Some((_, bn)) = WORD_OVERRIDES.iter().find(|(en, _)| *en == lower.as_str()) {
            return (*bn).to_string();
        }
    }

    // Multi-word: transliterate each whitespace-separated token.
    if input.contains(char::is_whitespace) {
        return input
            .split_inclusive(char::is_whitespace)
            .map(|part| {
                let trimmed = part.trim_end_matches(char::is_whitespace);
                let ws: String = part[trimmed.len()..].to_string();
                if trimmed.is_empty() {
                    part.to_string()
                } else {
                    format!("{}{}", transliterate_token(trimmed, config), ws)
                }
            })
            .collect();
    }

    transliterate_token(input, config)
}

fn transliterate_token(input: &str, config: &PhoneticConfig) -> String {
    if config.use_word_overrides {
        let lower = input.to_ascii_lowercase();
        if let Some((_, bn)) = WORD_OVERRIDES.iter().find(|(en, _)| *en == lower.as_str()) {
            return (*bn).to_string();
        }
        let trimmed_punct = lower.trim_end_matches(|c: char| c == '.' || c == ',' || c == '?' || c == '!' || c == ';');
        if trimmed_punct != lower && !trimmed_punct.is_empty() {
            if let Some((_, bn)) = WORD_OVERRIDES.iter().find(|(en, _)| *en == trimmed_punct) {
                let suffix = &input[trimmed_punct.len()..];
                let suffix_trans = transliterate_token(suffix, config);
                return format!("{}{}", bn, suffix_trans);
            }
        }
    }

    let mut out = String::with_capacity(input.len() * 3);
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0usize;
    let mut last_was_consonant = false;

    while i < chars.len() {
        let remaining: String = chars[i..].iter().collect();

        // Digits
        if let Some(c) = remaining.chars().next() {
            if c.is_ascii_digit() {
                if config.bengali_digits {
                    out.push(bengali_digit(c));
                } else {
                    out.push(c);
                }
                i += 1;
                last_was_consonant = false;
                continue;
            }
        }

        // Longest-match against rule table
        let mut matched = false;
        for rule in ORDERED_RULES.iter() {
            if !remaining.starts_with(rule.pattern) {
                // Case-insensitive fallback for lowercase patterns
                let rem_lower = remaining.to_ascii_lowercase();
                let pat_lower = rule.pattern.to_ascii_lowercase();
                if !rem_lower.starts_with(&pat_lower) {
                    continue;
                }
                // Only allow case-insensitive when pattern is all lowercase ASCII
                if rule.pattern.chars().any(|c| c.is_ascii_uppercase()) {
                    continue;
                }
                let adv = rule.pattern.chars().count();
                apply_replacement(
                    &mut out,
                    rule.replacement,
                    &mut last_was_consonant,
                    config,
                );
                i += adv;
                matched = true;
                break;
            }

            let adv = rule.pattern.chars().count();
            apply_replacement(
                &mut out,
                rule.replacement,
                &mut last_was_consonant,
                config,
            );
            i += adv;
            matched = true;
            break;
        }

        if !matched {
            // Pass through unknown character
            out.push(chars[i]);
            i += 1;
            last_was_consonant = false;
        }
    }

    // Post-process: attach vowel kars after consonants
    let shaped = shape_vowels(&out);
    let nfc: String = shaped.nfc().collect();
    nfc.replace("\u{09AF}\u{09BC}", "\u{09DF}")
        .replace("\u{09A1}\u{09BC}", "\u{09DC}")
        .replace("\u{09A2}\u{09BC}", "\u{09DD}")
}

fn apply_replacement(
    out: &mut String,
    replacement: &str,
    last_was_consonant: &mut bool,
    config: &PhoneticConfig,
) {
    if replacement == "." && !config.convert_danda {
        out.push('.');
        *last_was_consonant = false;
        return;
    }

    // Hasanta (halant) — mark for conjunct formation
    if replacement == HASANTA || replacement == "্‌" {
        out.push_str(replacement);
        *last_was_consonant = false;
        return;
    }

    if is_bengali_vowel(replacement) {
        if *last_was_consonant {
            let kar = to_kar(replacement);
            out.push_str(kar);
        } else {
            out.push_str(replacement);
        }
        *last_was_consonant = false;
        return;
    }

    if is_bengali_consonant(replacement) {
        out.push_str(replacement);
        *last_was_consonant = true;
        return;
    }

    // Signs like anusvara, visarga, chandrabindu attach without resetting
    // consonant state for anusvara/chandrabindu after consonants.
    match replacement {
        "ং" | "ঁ" | "ঃ" | "।"| "৳" => {
            out.push_str(replacement);
            *last_was_consonant = false;
        }
        _ => {
            out.push_str(replacement);
            *last_was_consonant = is_bengali_consonant(replacement);
        }
    }
}

/// Second pass: if independent vowels appear immediately after consonants
/// without kar conversion (edge cases), fix them.
fn shape_vowels(input: &str) -> String {
    // Primary shaping already happens in apply_replacement.
    // This pass collapses redundant অ after consonants (inherent vowel).
    let mut result = String::with_capacity(input.len());
    let graphemes: Vec<&str> = {
        use unicode_segmentation::UnicodeSegmentation;
        input.graphemes(true).collect()
    };

    for (idx, g) in graphemes.iter().enumerate() {
        if *g == "অ" && idx > 0 && is_bengali_consonant(graphemes[idx - 1]) {
            // Inherent vowel — omit explicit অ after consonant
            continue;
        }
        result.push_str(g);
    }
    result
}

fn bengali_digit(c: char) -> char {
    match c {
        '0' => '০',
        '1' => '১',
        '2' => '২',
        '3' => '৩',
        '4' => '৪',
        '5' => '৫',
        '6' => '৬',
        '7' => '৭',
        '8' => '৮',
        '9' => '৯',
        _ => c,
    }
}

fn should_preserve_latin(input: &str) -> bool {
    let lower = input.to_ascii_lowercase();
    lower.contains("://")
        || lower.contains('@') && lower.contains('.')
        || lower.starts_with("www.")
        || lower.starts_with("http")
        || (input.contains('\\') && input.contains(':'))
        || looks_like_code_token(input)
}

fn looks_like_code_token(input: &str) -> bool {
    // CamelCase or snake_case identifiers with mixed case / underscores
    if input.contains('_') && input.is_ascii() {
        return true;
    }
    let has_upper = input.chars().any(|c| c.is_ascii_uppercase());
    let has_lower = input.chars().any(|c| c.is_ascii_lowercase());
    has_upper && has_lower && input.chars().all(|c| c.is_ascii_alphanumeric())
}

/// Generate candidate Bengali interpretations for ambiguous input.
pub fn ambiguous_candidates(input: &str, config: &PhoneticConfig) -> Vec<String> {
    let primary = transliterate(input, config);
    let mut candidates = vec![primary.clone()];

    // Variant substitutions for common ambiguities
    let variants = generate_spelling_variants(input);
    for v in variants {
        let t = transliterate(&v, config);
        if !candidates.contains(&t) {
            candidates.push(t);
        }
    }

    candidates
}

fn generate_spelling_variants(input: &str) -> Vec<String> {
    let mut variants = Vec::new();
    let lower = input.to_ascii_lowercase();

    // b/v → ভ
    if lower.contains('v') {
        variants.push(lower.replace('v', "bh"));
    }
    if lower.contains("bh") {
        variants.push(lower.replace("bh", "v"));
    }
    // sh/s
    if lower.contains("sh") {
        variants.push(lower.replace("sh", "s"));
    }
    if lower.starts_with('s') && !lower.starts_with("sh") {
        variants.push(format!("sh{}", &lower[1..]));
    }
    // oo/u
    if lower.contains("oo") {
        variants.push(lower.replace("oo", "u"));
    }

    variants
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_words() {
        let cfg = PhoneticConfig::default();
        assert_eq!(transliterate("ami", &cfg), "আমি");
        assert_eq!(transliterate("tumi", &cfg), "তুমি");
        assert_eq!(transliterate("amar", &cfg), "আমার");
        assert_eq!(transliterate("bangla", &cfg), "বাংলা");
        assert_eq!(transliterate("bangladesh", &cfg), "বাংলাদেশ");
        assert_eq!(transliterate("bhalo", &cfg), "ভালো");
        assert_eq!(transliterate("valo", &cfg), "ভালো");
        assert_eq!(transliterate("shundor", &cfg), "সুন্দর");
        assert_eq!(transliterate("sundor", &cfg), "সুন্দর");
        assert_eq!(transliterate("valobasha", &cfg), "ভালোবাসা");
        assert_eq!(transliterate("kemon", &cfg), "কেমন");
    }

    #[test]
    fn sentence() {
        let cfg = PhoneticConfig::default();
        let out = transliterate("ami banglay likhte chai", &cfg);
        assert!(out.contains("আমি"));
        assert!(out.contains("লিখতে"));
        assert!(out.contains("চাই"));
    }

    #[test]
    fn preserve_url() {
        let cfg = PhoneticConfig::default();
        assert_eq!(
            transliterate("https://example.com", &cfg),
            "https://example.com"
        );
    }

    #[test]
    fn empty() {
        let cfg = PhoneticConfig::default();
        assert_eq!(transliterate("", &cfg), "");
    }
}
