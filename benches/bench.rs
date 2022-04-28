use std::{fs, io};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jsonformat::{format, format_reader_writer, Indentation};

fn criterion_benchmark(c: &mut Criterion) {
    let file = include_str!("large-file.json");

    c.bench_function("Format json default settings", |b| {
        b.iter(|| {
            let json = format(&file, Indentation::Default);
            black_box(json);
        })
    });

    c.bench_function("Format json custom indentation", |b| {
        b.iter(|| {
            let json = format(&file, Indentation::Custom("123456"));
            black_box(json);
        })
    });

    c.bench_function("Format json no utf8 validation", |b| {
        b.iter(|| {
            let mut writer = Vec::with_capacity(file.len() * 2);

            format_reader_writer(file.as_bytes(), &mut writer, Indentation::Default).unwrap();
            black_box(writer);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
