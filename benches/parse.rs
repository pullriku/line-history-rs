use criterion::{criterion_group, criterion_main};
use line_history::history::ignore_errors;

fn bench_parse(criterion: &mut criterion::Criterion) {
    let mut group = criterion.benchmark_group("size");

    let src = std::fs::read_to_string("./history.txt").unwrap();
    group.bench_function("parse", |b| {
        b.iter(|| {
            let _history = line_history::parse::parse_history(&src);
        })
    });

    group.bench_function("parse into_owned", |b| {
        b.iter(|| {
            let history = line_history::parse::parse_history(&src);
            let history = ignore_errors(history);
            let _ = history.into_owned();
        });
    });

    group.bench_function("clone into_owned", |b| {
        let history = line_history::parse::parse_history(&src);
        let history = ignore_errors(history);
        b.iter(|| {
            let _ = history.clone().into_owned();
        });
    });
}

criterion_group!(benches, bench_parse);
criterion_main!(benches);
