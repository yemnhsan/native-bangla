//! Composition state machine for Native Bangla phonetic input.
//!
//! Maintains an intermediate Latin buffer and commits Bengali Unicode
//! when the user confirms a word (space, punctuation, candidate accept).

use crate::transliterate::{ambiguous_candidates, transliterate, PhoneticConfig};
use serde::{Deserialize, Serialize};

/// Input mode for the composition session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum InputMode {
    #[default]
    Phonetic,
    Traditional,
    English,
}

/// Events that drive the composition state machine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompositionEvent {
    /// A printable character was typed.
    Char(char),
    /// Backspace
    Backspace,
    /// Delete (forward)
    Delete,
    /// Commit current composition (Space / Enter / candidate accept)
    Commit,
    /// Cancel composition (Escape)
    Cancel,
    /// Accept a specific candidate string
    AcceptCandidate(String),
    /// Force English for the current word
    ForceEnglish,
    /// Switch input mode
    SetMode(InputMode),
}

/// Result of processing an event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompositionUpdate {
    /// Current phonetic (Latin) buffer.
    pub raw_buffer: String,
    /// Current Bengali composition preview.
    pub composed: String,
    /// Text to insert/commit into the host application (if any).
    pub commit_text: Option<String>,
    /// Whether composition is still active.
    pub composing: bool,
    /// Candidate strings for the suggestion UI.
    pub candidates: Vec<String>,
}

/// Active composition session.
#[derive(Debug, Clone)]
pub struct CompositionSession {
    raw: String,
    mode: InputMode,
    config: PhoneticConfig,
    force_english: bool,
    composing: bool,
}

impl Default for CompositionSession {
    fn default() -> Self {
        Self::new(PhoneticConfig::default())
    }
}

impl CompositionSession {
    pub fn new(config: PhoneticConfig) -> Self {
        Self {
            raw: String::new(),
            mode: InputMode::Phonetic,
            config,
            force_english: false,
            composing: false,
        }
    }

    pub fn mode(&self) -> InputMode {
        self.mode
    }

    pub fn is_composing(&self) -> bool {
        self.composing
    }

    pub fn raw_buffer(&self) -> &str {
        &self.raw
    }

    pub fn preview(&self) -> String {
        self.compose_preview()
    }

    pub fn process(&mut self, event: CompositionEvent) -> CompositionUpdate {
        match event {
            CompositionEvent::SetMode(mode) => {
                // Commit or cancel before switching
                let commit = if self.composing && !self.raw.is_empty() {
                    Some(self.compose_preview())
                } else {
                    None
                };
                self.reset_buffers();
                self.mode = mode;
                self.force_english = false;
                return CompositionUpdate {
                    raw_buffer: String::new(),
                    composed: String::new(),
                    commit_text: commit,
                    composing: false,
                    candidates: vec![],
                };
            }
            CompositionEvent::ForceEnglish => {
                self.force_english = true;
                return self.snapshot(None);
            }
            CompositionEvent::Cancel => {
                self.reset_buffers();
                return CompositionUpdate {
                    raw_buffer: String::new(),
                    composed: String::new(),
                    commit_text: None,
                    composing: false,
                    candidates: vec![],
                };
            }
            CompositionEvent::Commit => {
                let text = if self.raw.is_empty() {
                    None
                } else {
                    Some(self.compose_preview())
                };
                self.reset_buffers();
                return CompositionUpdate {
                    raw_buffer: String::new(),
                    composed: String::new(),
                    commit_text: text,
                    composing: false,
                    candidates: vec![],
                };
            }
            CompositionEvent::AcceptCandidate(text) => {
                self.reset_buffers();
                return CompositionUpdate {
                    raw_buffer: String::new(),
                    composed: String::new(),
                    commit_text: Some(text),
                    composing: false,
                    candidates: vec![],
                };
            }
            CompositionEvent::Backspace => {
                if self.raw.is_empty() {
                    return CompositionUpdate {
                        raw_buffer: String::new(),
                        composed: String::new(),
                        commit_text: None,
                        composing: false,
                        candidates: vec![],
                    };
                }
                self.raw.pop();
                if self.raw.is_empty() {
                    self.composing = false;
                }
                return self.snapshot(None);
            }
            CompositionEvent::Delete => {
                // Composition buffer has no forward cursor yet — no-op
                return self.snapshot(None);
            }
            CompositionEvent::Char(c) => {
                return self.handle_char(c);
            }
        }
    }

    fn handle_char(&mut self, c: char) -> CompositionUpdate {
        // English mode or forced English: pass through
        if self.mode == InputMode::English || self.force_english {
            self.force_english = false;
            return CompositionUpdate {
                raw_buffer: String::new(),
                composed: String::new(),
                commit_text: Some(c.to_string()),
                composing: false,
                candidates: vec![],
            };
        }

        if self.mode == InputMode::Traditional {
            // Traditional layouts are handled by key-mapping tables in the IME.
            // Here we treat as direct Unicode insertion placeholder.
            return CompositionUpdate {
                raw_buffer: String::new(),
                composed: String::new(),
                commit_text: Some(c.to_string()),
                composing: false,
                candidates: vec![],
            };
        }

        // Space / punctuation commits the current word
        if c.is_whitespace() || is_commit_punctuation(c) {
            let mut commit = self.compose_preview();
            if c == ' ' {
                commit.push(' ');
            } else if c == '.' {
                // Prefer danda when converting
                if self.config.convert_danda {
                    commit.push('।');
                } else {
                    commit.push('.');
                }
            } else {
                commit.push(c);
            }
            self.reset_buffers();
            return CompositionUpdate {
                raw_buffer: String::new(),
                composed: String::new(),
                commit_text: Some(commit),
                composing: false,
                candidates: vec![],
            };
        }

        self.raw.push(c);
        self.composing = true;
        self.snapshot(None)
    }

    fn compose_preview(&self) -> String {
        if self.raw.is_empty() {
            return String::new();
        }
        if self.mode == InputMode::English || self.force_english {
            return self.raw.clone();
        }
        transliterate(&self.raw, &self.config)
    }

    fn candidates_for_buffer(&self) -> Vec<String> {
        if self.raw.is_empty() || self.mode != InputMode::Phonetic {
            return vec![];
        }
        ambiguous_candidates(&self.raw, &self.config)
            .into_iter()
            .take(5)
            .collect()
    }

    fn snapshot(&self, commit: Option<String>) -> CompositionUpdate {
        CompositionUpdate {
            raw_buffer: self.raw.clone(),
            composed: self.compose_preview(),
            commit_text: commit,
            composing: self.composing,
            candidates: self.candidates_for_buffer(),
        }
    }

    fn reset_buffers(&mut self) {
        self.raw.clear();
        self.composing = false;
        self.force_english = false;
    }
}

fn is_commit_punctuation(c: char) -> bool {
    matches!(
        c,
        '.' | ',' | ';' | ':' | '!' | '?' | ')' | ']' | '}' | '|' | '"' | '\''
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compose_ami() {
        let mut s = CompositionSession::default();
        for c in ['a', 'm', 'i'] {
            let u = s.process(CompositionEvent::Char(c));
            assert!(u.composing);
        }
        assert_eq!(s.preview(), "আমি");
        let u = s.process(CompositionEvent::Commit);
        assert_eq!(u.commit_text.as_deref(), Some("আমি"));
        assert!(!u.composing);
    }

    #[test]
    fn backspace() {
        let mut s = CompositionSession::default();
        s.process(CompositionEvent::Char('a'));
        s.process(CompositionEvent::Char('m'));
        s.process(CompositionEvent::Backspace);
        assert_eq!(s.raw_buffer(), "a");
    }

    #[test]
    fn space_commits_with_space() {
        let mut s = CompositionSession::default();
        for c in "ami".chars() {
            s.process(CompositionEvent::Char(c));
        }
        let u = s.process(CompositionEvent::Char(' '));
        assert_eq!(u.commit_text.as_deref(), Some("আমি "));
    }

    #[test]
    fn mode_switch_commits() {
        let mut s = CompositionSession::default();
        for c in "ami".chars() {
            s.process(CompositionEvent::Char(c));
        }
        let u = s.process(CompositionEvent::SetMode(InputMode::English));
        assert_eq!(u.commit_text.as_deref(), Some("আমি"));
        assert_eq!(s.mode(), InputMode::English);
    }

    #[test]
    fn english_passthrough() {
        let mut s = CompositionSession::default();
        s.process(CompositionEvent::SetMode(InputMode::English));
        let u = s.process(CompositionEvent::Char('x'));
        assert_eq!(u.commit_text.as_deref(), Some("x"));
        assert!(!u.composing);
    }
}
