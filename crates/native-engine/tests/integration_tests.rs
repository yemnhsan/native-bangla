use dictionary_core::Dictionary;
use native_engine::NativeEngine;
use prediction_engine::PredictRequest;
use std::path::PathBuf;
use user_preferences::NativeBanglaPreferences;

#[test]
fn test_phonetic_transliteration_common_words() {
    let engine = NativeEngine::new(NativeBanglaPreferences::default());

    let cases = [
        ("ami", "আমি"),
        ("tumi", "তুমি"),
        ("apni", "আপনি"),
        ("bangla", "বাংলা"),
        ("bangladesh", "বাংলাদেশ"),
        ("valobasha", "ভালোবাসা"),
        ("dhonnobad", "ধন্যবাদ"),
        ("shundor", "সুন্দর"),
    ];

    for (input, expected) in cases {
        assert_eq!(
            engine.transliterate(input),
            expected,
            "Transliteration failed for '{}'",
            input
        );
    }
}

#[test]
fn test_sentence_and_danda_conversion() {
    let mut prefs = NativeBanglaPreferences::default();
    prefs.typing.convert_danda = true;
    let engine = NativeEngine::new(prefs);

    let output = engine.transliterate("ami banglay gaan gai.");
    assert_eq!(output, "আমি বাংলায় গান গাই।");
}

#[test]
fn test_bengali_digits_conversion() {
    let mut prefs = NativeBanglaPreferences::default();
    prefs.typing.bengali_digits = true;
    let engine = NativeEngine::new(prefs);

    let output = engine.transliterate("1234567890");
    assert_eq!(output, "১২৩৪৫৬৭৮৯০");
}

#[test]
fn test_mixed_language_preservation() {
    let engine = NativeEngine::new(NativeBanglaPreferences::default());

    // URLs, emails, and camelCase should be preserved as English
    assert_eq!(engine.transliterate("https://nativebangla.org"), "https://nativebangla.org");
    assert_eq!(engine.transliterate("info@nativebangla.org"), "info@nativebangla.org");
    assert_eq!(engine.transliterate("myVariableName"), "myVariableName");
}

#[test]
fn test_prediction_with_seed_dictionary() {
    let dict_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../resources/dictionaries/bn-core-seed.json");
    assert!(dict_path.exists(), "Seed dictionary must exist");

    let dict = Dictionary::load(&dict_path).expect("Failed to load seed dictionary");
    let engine = NativeEngine::new(NativeBanglaPreferences::default()).with_dictionary(dict);

    let req = PredictRequest {
        phonetic_prefix: "ban".into(),
        context_words: vec![],
        limit: 5,
        next_word: false,
    };

    let res = engine.predict(&req);
    assert!(!res.suggestions.is_empty(), "Expected suggestions for prefix 'ban'");

    let suggestion_texts: Vec<&str> = res.suggestions.iter().map(|s| s.text.as_str()).collect();
    assert!(
        suggestion_texts.iter().any(|&s| s == "বাংলা" || s == "বাংলাদেশ"),
        "Expected 'বাংলা' or 'বাংলাদেশ' in suggestions, got: {:?}",
        suggestion_texts
    );
}

#[test]
fn test_next_word_prediction() {
    let dict_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../resources/dictionaries/bn-core-seed.json");
    let dict = Dictionary::load(&dict_path).expect("Failed to load seed dictionary");
    let engine = NativeEngine::new(NativeBanglaPreferences::default()).with_dictionary(dict);

    let req = PredictRequest {
        phonetic_prefix: "".into(),
        context_words: vec!["আমার".into()],
        limit: 5,
        next_word: true,
    };

    let res = engine.predict(&req);
    assert!(!res.suggestions.is_empty(), "Expected next-word suggestions after 'আমার'");
}
