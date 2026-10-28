//! Compact prefix trie for Bengali / Latin dictionary lookup.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A dictionary entry with frequency weight.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DictEntry {
    pub word: String,
    /// Higher = more common. Arbitrary positive scale.
    pub frequency: u32,
    /// Optional phonetic latin key for reverse lookup.
    #[serde(default)]
    pub phonetic: Option<String>,
}

/// Trie node.
#[derive(Debug, Default, Clone)]
struct Node {
    children: BTreeMap<char, Node>,
    /// Terminal marker with frequency (None = not a word end).
    terminal: Option<u32>,
}

/// Prefix-searchable dictionary index.
#[derive(Debug, Default, Clone)]
pub struct Trie {
    root: Node,
    len: usize,
}

impl Trie {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn insert(&mut self, word: &str, frequency: u32) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            node = node.children.entry(ch).or_default();
        }
        if node.terminal.is_none() {
            self.len += 1;
        }
        node.terminal = Some(frequency.max(node.terminal.unwrap_or(0)));
    }

    pub fn contains(&self, word: &str) -> bool {
        self.find_node(word).and_then(|n| n.terminal).is_some()
    }

    /// Prefix search returning up to `limit` entries, ranked by frequency desc.
    pub fn prefix_search(&self, prefix: &str, limit: usize) -> Vec<DictEntry> {
        let Some(node) = self.find_node(prefix) else {
            return vec![];
        };
        let mut results = Vec::new();
        let mut buffer = prefix.to_string();
        Self::collect(node, &mut buffer, &mut results, limit.saturating_mul(4));
        results.sort_by(|a, b| b.frequency.cmp(&a.frequency).then(a.word.cmp(&b.word)));
        results.truncate(limit);
        results
    }

    fn find_node(&self, key: &str) -> Option<&Node> {
        let mut node = &self.root;
        for ch in key.chars() {
            node = node.children.get(&ch)?;
        }
        Some(node)
    }

    fn collect(node: &Node, buffer: &mut String, out: &mut Vec<DictEntry>, max: usize) {
        if out.len() >= max {
            return;
        }
        if let Some(freq) = node.terminal {
            out.push(DictEntry {
                word: buffer.clone(),
                frequency: freq,
                phonetic: None,
            });
        }
        for (ch, child) in &node.children {
            if out.len() >= max {
                return;
            }
            buffer.push(*ch);
            Self::collect(child, buffer, out, max);
            buffer.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefix() {
        let mut t = Trie::new();
        t.insert("আমি", 1000);
        t.insert("আমার", 900);
        t.insert("আমাদের", 800);
        t.insert("আপনি", 700);

        let r = t.prefix_search("আম", 5);
        assert_eq!(r[0].word, "আমি");
        assert!(r.iter().any(|e| e.word == "আমার"));
        assert!(!r.iter().any(|e| e.word == "আপনি"));
    }
}
