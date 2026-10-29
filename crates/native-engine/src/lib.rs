//! # native-engine
//!
//! High-level orchestration API used by the Tauri desktop app and tests.
//! The Windows IME talks to the same linguistic stack through `ime-bridge`.

#![deny(unsafe_code)]

use dictionary_core::Dictionary;
use phonetic_engine::{
    transliterate, CompositionEvent, CompositionSession, InputMode, PhoneticConfig,
};
use prediction_engine::{PredictRequest, PredictResponse, Predictor};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use user_preferences::NativeBanglaPreferences;

pub const PRODUCT_NAME: &str = "Native Bangla";
pub const ENGINE_NAME: &str = "Native Engine";
pub const PREDICT_NAME: &str = "Native Predict";
pub const TAGLINE: &str = "Type Naturally. Write Beautifully.";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("dictionary: {0}")]
    Dictionary(#[from] dictionary_core::DictionaryError),
    #[error("preferences: {0}")]
    Prefs(#[from] user_preferences::PrefsError),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineStatus {
    pub product: String,
    pub engine: String,
    pub predict: String,
    pub version: String,
    pub mode: InputMode,
    pub dictionary_words: usize,
    pub suggestions_enabled: bool,
}

/// In-process Native Engine instance (settings app / tests).
pub struct NativeEngine {
    pub session: CompositionSession,
    pub predictor: Option<Predictor>,
    pub prefs: NativeBanglaPreferences,
}

impl NativeEngine {
    pub fn new(prefs: NativeBanglaPreferences) -> Self {
        let mut config = PhoneticConfig::default();
        config.bengali_digits = prefs.typing.bengali_digits;
        config.convert_danda = prefs.typing.convert_danda;
        let mode = match prefs.typing.default_language {
            user_preferences::InputLanguage::English => InputMode::English,
            user_preferences::InputLanguage::BanglaTraditional => InputMode::Traditional,
            user_preferences::InputLanguage::BanglaPhonetic => InputMode::Phonetic,
        };
        let mut session = CompositionSession::new(config);
        session.process(CompositionEvent::SetMode(mode));
        Self {
            session,
            predictor: None,
            prefs,
        }
    }

    pub fn with_dictionary(mut self, dict: Dictionary) -> Self {
        let mut predictor = Predictor::new(dict);
        predictor.config.phonetic.bengali_digits = self.prefs.typing.bengali_digits;
        predictor.personal.learning_enabled = self.prefs.personalization.learning_enabled;
        predictor.config.enable_next_word = self.prefs.suggestions.next_word;
        self.predictor = Some(predictor);
        self
    }

    pub fn load_dictionary_path(self, path: impl AsRef<std::path::Path>) -> Result<Self, EngineError> {
        let dict = Dictionary::load(path)?;
        Ok(self.with_dictionary(dict))
    }

    pub fn status(&self) -> EngineStatus {
        EngineStatus {
            product: PRODUCT_NAME.into(),
            engine: ENGINE_NAME.into(),
            predict: PREDICT_NAME.into(),
            version: VERSION.into(),
            mode: self.session.mode(),
            dictionary_words: self
                .predictor
                .as_ref()
                .map(|p| p.dictionary.word_count())
                .unwrap_or(0),
            suggestions_enabled: self.prefs.suggestions.enabled,
        }
    }

    pub fn transliterate(&self, input: &str) -> String {
        transliterate(input, &PhoneticConfig {
            bengali_digits: self.prefs.typing.bengali_digits,
            convert_danda: self.prefs.typing.convert_danda,
            ..PhoneticConfig::default()
        })
    }

    pub fn predict(&self, req: &PredictRequest) -> PredictResponse {
        if let Some(ref p) = self.predictor {
            p.predict(req)
        } else {
            let primary = self.transliterate(&req.phonetic_prefix);
            PredictResponse {
                suggestions: vec![prediction_engine::Suggestion {
                    text: primary.clone(),
                    score: 1.0,
                    source: prediction_engine::SuggestionSource::Phonetic,
                }],
                primary,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity() {
        let engine = NativeEngine::new(NativeBanglaPreferences::default());
        let s = engine.status();
        assert_eq!(s.product, "Native Bangla");
        assert_eq!(s.engine, "Native Engine");
        assert_eq!(engine.transliterate("ami"), "আমি");
    }
}
