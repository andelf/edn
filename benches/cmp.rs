use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../data/docs.edn");
    let v1 = edn::parser::parse_edn(input).unwrap();
    let v2 = edn::parser::parse_edn(input).unwrap();

    c.bench_function("equal", |b| b.iter(|| assert_eq!(v1, v2)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
