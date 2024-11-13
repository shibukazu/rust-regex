use criterion::{criterion_group, criterion_main, Criterion};
use rust_regex::do_matching;
use std::time::Duration;

const INPUTS: &[(&str, &str, &str)] = &[
    ("n = 2", "a?a?aa", "aa"),
    ("n = 4", "a?a?a?a?aaaa", "aaaa"),
    ("n = 8", "a?a?a?a?a?a?a?a?aaaaaaaa", "aaaaaaaa"),
    (
        "n = 16",
        "a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaaaaaa",
        "aaaaaaaaaaaaaaaa",
    ),
];

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("regex");
    group.measurement_time(Duration::from_secs(12));

    for i in INPUTS {
        group.bench_with_input(i.0, &(i.1, i.2), |b, (expr, line)| {
            b.iter(|| do_matching(expr, line));
        });
    }
}

criterion_group!(benches, bench);
criterion_main!(benches);
