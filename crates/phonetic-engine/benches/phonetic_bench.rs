use criterion::{black_box, criterion_group, criterion_main, Criterion};
use phonetic_engine::{transliterate, PhoneticConfig};

fn bench_common_words(c: &mut Criterion) {
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

    c.bench_function("transliterate_common_words", |b| {
        b.iter(|| {
            for w in &words {
                black_box(transliterate(w, &cfg));
            }
        })
    });
}

fn bench_long_input(c: &mut Criterion) {
    let cfg = PhoneticConfig::default();
    let long = "ami ajke khub bhalo achi ebong banglay likhte chai karon bangla amar priyo bhasha";
    c.bench_function("transliterate_long_sentence", |b| {
        b.iter(|| black_box(transliterate(long, &cfg)))
    });
}

criterion_group!(benches, bench_common_words, bench_long_input);
criterion_main!(benches);
