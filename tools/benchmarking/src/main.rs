//! Micro-benchmark harness printing p50/p95/p99 for Native Bangla engines.

use dictionary_core::Dictionary;
use phonetic_engine::{transliterate, PhoneticConfig};
use prediction_engine::{PredictRequest, Predictor};
use std::path::PathBuf;
use std::time::Instant;

fn percentiles(mut samples: Vec<f64>) -> (f64, f64, f64) {
    samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let pct = |p: f64| {
        let idx = ((p / 100.0) * (samples.len() as f64 - 1.0)).round() as usize;
        samples[idx]
    };
    (pct(50.0), pct(95.0), pct(99.0))
}

fn main() -> anyhow::Result<()> {
    let cfg = PhoneticConfig::default();
    let words = [
        "ami",
        "tumi",
        "bangla",
        "bangladesh",
        "valobasha",
        "shundor",
        "kemon",
        "ami banglay likhte chai",
    ];

    let mut samples = Vec::with_capacity(2000);
    for _ in 0..2000 {
        let start = Instant::now();
        for w in &words {
            let _ = transliterate(w, &cfg);
        }
        samples.push(start.elapsed().as_secs_f64() * 1000.0 / words.len() as f64);
    }
    let (p50, p95, p99) = percentiles(samples);
    println!("Native Engine transliteration (ms/word): p50={p50:.4} p95={p95:.4} p99={p99:.4}");

    let dict_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../resources/dictionaries/bn-core-seed.json");
    if dict_path.exists() {
        let dict = Dictionary::load(&dict_path)?;
        let predictor = Predictor::new(dict);
        let mut samples = Vec::with_capacity(1000);
        for _ in 0..1000 {
            let start = Instant::now();
            let _ = predictor.predict(&PredictRequest {
                phonetic_prefix: "am".into(),
                context_words: vec![],
                limit: 5,
                next_word: false,
            });
            samples.push(start.elapsed().as_secs_f64() * 1000.0);
        }
        let (p50, p95, p99) = percentiles(samples);
        println!("Native Predict prefix 'am' (ms): p50={p50:.4} p95={p95:.4} p99={p99:.4}");
    } else {
        println!("Dictionary not found at {}, skipping predict bench", dict_path.display());
    }

    Ok(())
}
