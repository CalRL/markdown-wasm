use std::time::{Duration, Instant};

use markdown_wasm::parse_markdown; // change this

fn main() {
    let samples = vec![
        include_str!("../../benches/samples/small.md"),
        include_str!("../../benches/samples/medium.md"),
        include_str!("../../benches/samples/large.md"),
        include_str!("../../benches/samples/huge.md"),
    ];

    let duration = Duration::from_secs(5);
    let start = Instant::now();

    let mut operations = 0usize;

    while start.elapsed() < duration {
        for sample in &samples {
            let html = parse_markdown(*sample);
            std::hint::black_box(html);
            operations += 1;
        }
    }

    let elapsed = start.elapsed().as_secs_f64();
    let ops_per_second = operations as f64 / elapsed;

    let total_bytes: usize = samples.iter().map(|s| s.len()).sum();
    let mb_per_sec = (operations * total_bytes) as f64 / elapsed / 1_000_000.0;

    println!("MB/sec: {:.2}", mb_per_sec);
    println!("operations: {operations}");
    println!("elapsed: {:.3}s", elapsed);
    println!("ops/sec: {:.2}", ops_per_second);
}