use criterion::{criterion_group, criterion_main, Criterion};
use jsonformat::{format_json, Indentation};
use std::{fs, io};

/// You need a json file called massive.json in your project root
fn format_massive_json(file: &str) -> io::Result<String> {
    Ok(format_json(&file, Indentation::Default))
}

fn criterion_benchmark(c: &mut Criterion) {
    let file = fs::read_to_string("massive.json").expect("massive.json file in project directory");

    c.bench_function("Format massive json", |b| {
        b.iter(|| format_massive_json(&file))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
