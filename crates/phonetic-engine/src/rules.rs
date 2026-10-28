//! Phonetic rule tables for Native Bangla.
//!
//! Rules follow widely documented Bengali phonetic conventions
//! (Avro-style orthography) without copying proprietary source.

use once_cell::sync::Lazy;
use std::collections::HashMap;

/// A single phonetic replacement rule.
#[derive(Debug, Clone)]
pub struct Rule {
    pub pattern: &'static str,
    pub replacement: &'static str,
}

/// Kar (dependent vowel) forms keyed by independent vowel letter.
pub static VOWEL_KAR: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    HashMap::from([
        ("অ", ""),
        ("আ", "া"),
        ("ই", "ি"),
        ("ঈ", "ী"),
        ("উ", "ু"),
        ("ঊ", "ূ"),
        ("ঋ", "ৃ"),
        ("এ", "ে"),
        ("ঐ", "ৈ"),
        ("ও", "ো"),
        ("ঔ", "ৌ"),
    ])
});

/// Consonant phonetic patterns (longest matches listed first in ORDERED_RULES).
pub static CONSONANTS: &[(&str, &str)] = &[
    ("ksh", "ক্ষ"),
    ("ggn", "জ্ঞ"),
    ("nj", "ঞ্জ"),
    ("ng", "ং"),
    ("chh", "ছ"),
    ("ch", "চ"),
    ("zh", "ঝ"),
    ("jh", "ঝ"),
    ("th", "থ"),
    ("Th", "ঠ"),
    ("dh", "ধ"),
    ("Dh", "ঢ"),
    ("ph", "ফ"),
    ("bh", "ভ"),
    ("sh", "শ"),
    ("Sh", "ষ"),
    ("rh", "ঢ়"),
    ("R", "ড়"),
    ("kh", "খ"),
    ("gh", "ঘ"),
    ("kk", "ক্ক"),
    ("tt", "ট্ট"),
    ("dd", "ড্ড"),
    ("pp", "প্প"),
    ("bb", "ব্ব"),
    ("mm", "ম্ম"),
    ("nn", "ন্ন"),
    ("ll", "ল্ল"),
    ("ss", "স্স"),
    ("k", "ক"),
    ("g", "গ"),
    ("c", "চ"),
    ("j", "জ"),
    ("T", "ট"),
    ("D", "ড"),
    ("t", "ত"),
    ("d", "দ"),
    ("n", "ন"),
    ("N", "ণ"),
    ("p", "প"),
    ("f", "ফ"),
    ("b", "ব"),
    ("v", "ভ"),
    ("m", "ম"),
    ("z", "জ"),
    ("r", "র"),
    ("l", "ল"),
    ("s", "স"),
    ("S", "শ"),
    ("h", "হ"),
    ("y", "য়"),
    ("Y", "য়"),
    ("w", "ও"),
    ("x", "ক্স"),
];

/// Independent / full vowel patterns.
pub static VOWELS: &[(&str, &str)] = &[
    ("ou", "ঔ"),
    ("oi", "ঐ"),
    ("oo", "উ"),
    ("ee", "ঈ"),
    ("aa", "আ"),
    ("a", "আ"),
    ("i", "ই"),
    ("I", "ঈ"),
    ("u", "উ"),
    ("U", "ঊ"),
    ("e", "এ"),
    ("E", "এ"),
    ("o", "ও"),
    ("O", "ও"),
    ("rri", "ঋ"),
];

/// Special symbols and punctuation.
pub static SPECIALS: &[(&str, &str)] = &[
    (".", "।"),
    ("..", "."),
    ("$", "৳"),
    (":", "ঃ"),
    ("^", "ঁ"),
    (",,", "্‌"),
    (",", "্"),
    ("0", "০"),
    ("1", "১"),
    ("2", "২"),
    ("3", "৩"),
    ("4", "৪"),
    ("5", "৫"),
    ("6", "৬"),
    ("7", "৭"),
    ("8", "৮"),
    ("9", "৯"),
];

/// Common whole-word overrides for natural spelling variants.
pub static WORD_OVERRIDES: &[(&str, &str)] = &[
    ("ami", "আমি"),
    ("tumi", "তুমি"),
    ("amar", "আমার"),
    ("apni", "আপনি"),
    ("apnar", "আপনার"),
    ("bangla", "বাংলা"),
    ("bangladesh", "বাংলাদেশ"),
    ("bangladeshi", "বাংলাদেশি"),
    ("bhalo", "ভালো"),
    ("valo", "ভালো"),
    ("shundor", "সুন্দর"),
    ("sundor", "সুন্দর"),
    ("valobasha", "ভালোবাসা"),
    ("bhalobasha", "ভালোবাসা"),
    ("kemon", "কেমন"),
    ("acho", "আছো"),
    ("achho", "আছো"),
    ("ase", "আছে"),
    ("ache", "আছে"),
    ("hobe", "হবে"),
    ("hobe.", "হবে।"),
    ("korbo", "করব"),
    ("kore", "করে"),
    ("kora", "করা"),
    ("likhte", "লিখতে"),
    ("chai", "চাই"),
    ("chai.", "চাই।"),
    ("ekta", "একটা"),
    ("ekti", "একটি"),
    ("aj", "আজ"),
    ("aaj", "আজ"),
    ("kal", "কাল"),
    ("khub", "খুব"),
    ("onek", "অনেক"),
    ("shikkha", "শিক্ষা"),
    ("shiksha", "শিক্ষা"),
    ("shadhinota", "স্বাধীনতা"),
    ("swadhinota", "স্বাধীনতা"),
    ("vul", "ভুল"),
    ("bhul", "ভুল"),
    ("dhonnobad", "ধন্যবাদ"),
    ("dhonnobad.", "ধন্যবাদ।"),
    ("assalamualaikum", "আসসালামু আলাইকুম"),
    ("namaskar", "নমস্কার"),
];

/// Ordered rules by descending pattern length for longest-match scan.
pub static ORDERED_RULES: Lazy<Vec<Rule>> = Lazy::new(|| {
    let mut rules: Vec<Rule> = Vec::new();

    for (p, r) in CONSONANTS.iter().chain(VOWELS.iter()).chain(SPECIALS.iter()) {
        rules.push(Rule {
            pattern: p,
            replacement: r,
        });
    }

    rules.sort_by(|a, b| b.pattern.len().cmp(&a.pattern.len()));
    rules
});

/// Returns true if `s` is a Bengali consonant letter (base form).
pub fn is_bengali_consonant(s: &str) -> bool {
    matches!(
        s,
        "ক" | "খ" | "গ" | "ঘ" | "ঙ"
            | "চ" | "ছ" | "জ" | "ঝ" | "ঞ"
            | "ট" | "ঠ" | "ড" | "ঢ" | "ণ"
            | "ত" | "থ" | "দ" | "ধ" | "ন"
            | "প" | "ফ" | "ব" | "ভ" | "ম"
            | "য" | "র" | "ল" | "শ" | "ষ" | "স" | "হ"
            | "ড়" | "ঢ়" | "য়" | "ৎ" | "ক্ষ" | "জ্ঞ"
    )
}

/// Returns true if `s` is an independent Bengali vowel.
pub fn is_bengali_vowel(s: &str) -> bool {
    matches!(
        s,
        "অ" | "আ" | "ই" | "ঈ" | "উ" | "ঊ" | "ঋ" | "এ" | "ঐ" | "ও" | "ঔ"
    )
}

/// Convert independent vowel to kar form when attached after a consonant.
pub fn to_kar(vowel: &str) -> &str {
    VOWEL_KAR.get(vowel).copied().unwrap_or(vowel)
}
