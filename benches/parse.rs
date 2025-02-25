use criterion::{criterion_group, criterion_main};

fn bench_parse(criterion: &mut criterion::Criterion) {
    let mut group = criterion.benchmark_group("size");

    let src = std::fs::read_to_string("./history.txt").unwrap();
    group.bench_function("parse", |b| {
        b.iter(|| {
            let _history = line_history::parse::parse_history(&src);
        })
    });
}

criterion_group!(benches, bench_parse);
criterion_main!(benches);
