//! # prediction-engine (Native Predict)
//!
//! Dictionary-backed word completion, ranking, next-word prediction,
//! and privacy-preserving personalization for Native Bangla.

#![deny(unsafe_code)]

pub mod ngram;
pub mod personal;
pub mod predictor;
pub mod ranker;

pub use ngram::{seed_bengali_model, NGramModel};
pub use personal::{PersonalError, PersonalStore};
pub use predictor::{
    PredictRequest, PredictResponse, Predictor, PredictorConfig, Suggestion, SuggestionSource,
};
pub use ranker::{rank_candidates, RankWeights, ScoredCandidate};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod identity {
    pub const NAME: &str = "Native Predict";
}
