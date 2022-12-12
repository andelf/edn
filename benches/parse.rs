use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../data/docs.edn");
    c.bench_function("parse", |b| {
        b.iter(|| edn::parser::parse_edn(input).unwrap())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
