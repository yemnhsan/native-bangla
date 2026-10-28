//! # dictionary-core
//!
//! Dictionary formats, normalization, and prefix indexes for Native Bangla.

#![deny(unsafe_code)]

pub mod dictionary;
pub mod trie;

pub use dictionary::{Dictionary, DictionaryDocument, DictionaryError};
pub use trie::{DictEntry, Trie};
