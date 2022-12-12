use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../data/docs.edn");
    let v = edn::parser::parse_edn(input).unwrap();

    c.bench_function("dump", |b| b.iter(|| v.to_string()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
