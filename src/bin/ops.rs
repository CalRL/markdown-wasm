use std::{
    fs,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

use markdown_wasm::parse_markdown; // change this

struct BenchResult {
    name: String,
    bytes: usize,
    operations: usize,
    elapsed: f64,
    ops_per_sec: f64,
    mb_per_sec: f64,
}


fn collect_markdown_files(dir: impl AsRef<Path>) -> std::io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().is_some_and(|ext| ext == "md") {
            files.push(path);
        }
    }

    files.sort();
    Ok(files)
}

fn main() -> std::io::Result<()> {
    let files = collect_markdown_files("samples")?;

    if files.is_empty() {
        eprintln!("No .md files found in samples/");
        return Ok(());
    }

    let samples: Vec<(PathBuf, String)> = files
        .into_iter()
        .map(|path| {
            let content = fs::read_to_string(&path)?;
            Ok((path, content))
        })
        .collect::<std::io::Result<_>>()?;

    println!("Loaded {} markdown samples:", samples.len());

    let duration = Duration::from_secs(2);
    let mut results = Vec::new();

    for (path, sample) in &samples {
        let start = Instant::now();

        let mut operations = 0usize;
        let mut bytes_processed = 0usize;

        while start.elapsed() < duration {
            let html = parse_markdown(std::hint::black_box(sample));
            std::hint::black_box(html);

            operations += 1;
            bytes_processed += sample.len();
        }

        let elapsed = start.elapsed().as_secs_f64();
        let ops_per_sec = operations as f64 / elapsed;
        let mb_per_sec = bytes_processed as f64 / elapsed / 1_000_000.0;

        results.push(BenchResult {
            name: path.display().to_string(),
            bytes: sample.len(),
            operations,
            elapsed,
            ops_per_sec,
            mb_per_sec,
        });
    }

    println!();

    for result in &results {
        println!("{}", result.name);
        println!("  size: {} bytes", result.bytes);
        println!("  operations: {}", result.operations);
        println!("  elapsed: {:.3}s", result.elapsed);
        println!("  ops/sec: {:.2}", result.ops_per_sec);
        println!("  MB/sec: {:.2}", result.mb_per_sec);
        println!();
    }

    let average_ops_per_sec =
        results.iter().map(|r| r.ops_per_sec).sum::<f64>() / results.len() as f64;

    let average_mb_per_sec =
        results.iter().map(|r| r.mb_per_sec).sum::<f64>() / results.len() as f64;

    println!("Average results across files:");
    println!("  avg ops/sec: {:.2}", average_ops_per_sec);
    println!("  avg MB/sec: {:.2}", average_mb_per_sec);

    Ok(())
}