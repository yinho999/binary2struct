use std::path::Path;

use binary2struct::{read_from_file, write_to_file};
use criterion::{criterion_group, criterion_main, Criterion};
use serde::{Deserialize, Serialize};
fn criterion_benchmark(c: &mut Criterion) {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Person {
        name: String,
        age: u8,
    }
    let file_path = Path::new("person.bin");
    c.bench_function("write_to_file", |b| {
        b.iter(|| {
            write_to_file(
                Person {
                    name: "John".to_string(),
                    age: 30,
                },
                file_path,
            )
        })
    });
    c.bench_function("read_from_file", |b| {
        b.iter(|| read_from_file::<Person>(file_path))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
