use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dictionary_core::{DictEntry, Dictionary, DictionaryDocument};
use prediction_engine::{PredictRequest, Predictor};

fn sample_dict() -> Dictionary {
    let mut entries = Vec::new();
    let words = [
        ("আমি", "ami", 1000u32),
        ("আমার", "amar", 900),
        ("আমাদের", "amader", 800),
        ("আপনি", "apni", 700),
        ("বাংলা", "bangla", 950),
        ("বাংলাদেশ", "bangladesh", 850),
        ("ভালো", "bhalo", 800),
        ("সুন্দর", "shundor", 600),
        ("কেমন", "kemon", 700),
    ];
    for (w, p, f) in words {
        entries.push(DictEntry {
            word: w.into(),
            frequency: f,
            phonetic: Some(p.into()),
        });
    }
    // Pad with synthetic entries for scale
    for i in 0..5000 {
        entries.push(DictEntry {
            word: format!("শব্দ{i}"),
            frequency: (i % 100) as u32,
            phonetic: Some(format!("shobdo{i}")),
        });
    }
    Dictionary::from_document(DictionaryDocument {
        version: 1,
        name: "bench".into(),
        locale: "bn".into(),
        license: "MIT".into(),
        source: "bench".into(),
        entries,
    })
}

fn bench_prefix(c: &mut Criterion) {
    let p = Predictor::new(sample_dict());
    c.bench_function("predict_prefix_am", |b| {
        b.iter(|| {
            black_box(p.predict(&PredictRequest {
                phonetic_prefix: "am".into(),
                context_words: vec![],
                limit: 5,
                next_word: false,
            }))
        })
    });
}

fn bench_next_word(c: &mut Criterion) {
    let p = Predictor::new(sample_dict());
    c.bench_function("predict_next_word", |b| {
        b.iter(|| {
            black_box(p.predict(&PredictRequest {
                phonetic_prefix: String::new(),
                context_words: vec!["আমি".into(), "আজ".into()],
                limit: 5,
                next_word: true,
            }))
        })
    });
}

criterion_group!(benches, bench_prefix, bench_next_word);
criterion_main!(benches);
