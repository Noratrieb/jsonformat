use std::{fs, io, path::PathBuf};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jsonformat::{format, format_reader_writer, Indentation};

fn criterion_benchmark(c: &mut Criterion) {
    // using `include_str` makes the benches a lot less reliable for some reason???
    let file = PathBuf::from(file!())
        .parent()
        .unwrap()
        .join("large-file.json");
    let file = fs::read_to_string(file).unwrap();

    c.bench_function("Format json default settings", |b| {
        b.iter(|| {
            let json = format(black_box(&file), Indentation::TwoSpace);
            black_box(json);
        })
    });

    c.bench_function("Format json custom indentation", |b| {
        b.iter(|| {
            let json = format(black_box(&file), Indentation::Custom("123456"));
            black_box(json);
        })
    });

    c.bench_function("Format json no utf8 validation", |b| {
        b.iter(|| {
            let mut writer = Vec::with_capacity(file.len() * 2);

            format_reader_writer(
                black_box(file.as_bytes()),
                &mut writer,
                Indentation::TwoSpace,
            )
            .unwrap();
            black_box(writer);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
