//! Reproducible dictionary build pipeline for Native Bangla.

use anyhow::{bail, Context, Result};
use clap::Parser;
use dictionary_core::DictionaryDocument;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "dictionary-builder", about = "Build Native Bangla dictionary artifacts")]
struct Args {
    /// Input dictionary JSON
    #[arg(long)]
    input: PathBuf,

    /// Output normalized dictionary JSON
    #[arg(long)]
    output: PathBuf,

    /// Override dictionary name
    #[arg(long)]
    name: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut doc = DictionaryDocument::load_path(&args.input)
        .with_context(|| format!("load {}", args.input.display()))?;
    doc = doc.normalize();
    if let Some(name) = args.name {
        doc.name = name;
    }
    doc.validate()
        .map_err(|e| anyhow::anyhow!("validation failed: {e}"))?;
    if let Some(parent) = args.output.parent() {
        std::fs::create_dir_all(parent)?;
    }
    doc.save_path(&args.output)
        .with_context(|| format!("save {}", args.output.display()))?;

    println!(
        "Native Bangla dictionary: {} entries → {}",
        doc.entries.len(),
        args.output.display()
    );

    // Sanity: must contain core words
    let words: std::collections::HashSet<_> = doc.entries.iter().map(|e| e.word.as_str()).collect();
    for required in ["আমি", "বাংলা", "বাংলাদেশ"] {
        if !words.contains(required) {
            bail!("missing required word: {required}");
        }
    }
    Ok(())
}
