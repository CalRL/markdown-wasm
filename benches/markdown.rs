use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;
use markdown_wasm::parse_blocks_only; // change this to your crate/module

fn bench_parse_markdown(c: &mut Criterion) {
    let samples = vec![
        ("small", include_str!("../samples/small.md")),
        ("medium", include_str!("../samples/medium.md")),
        ("large", include_str!("../samples/large.md")),
    ];

    let mut group = c.benchmark_group("parse_markdown");

    for (name, markdown) in samples {
        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            markdown,
            |b, input| {
                b.iter(|| {
                    let html = parse_blocks_only(black_box(input));
                    black_box(html);
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_parse_markdown);
criterion_main!(benches);