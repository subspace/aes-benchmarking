use criterion::{criterion_group, criterion_main, Criterion};
use aes_benchmarks::*;

pub fn criterion_benchmark(c: &mut Criterion) {

    let keys = [
      random_bytes_16(),
      random_bytes_16(),
      random_bytes_16(),
      random_bytes_16(),
      random_bytes_16(),
      random_bytes_16(),
      random_bytes_16(),
      random_bytes_16(),
      random_bytes_16(),
      random_bytes_16(),
      random_bytes_16(),
      random_bytes_16(),
      random_bytes_16(),
      random_bytes_16(),
      random_bytes_16(),
      random_bytes_16(),
      random_bytes_16(),
      random_bytes_16(),
      random_bytes_16(),
      random_bytes_16(),
      random_bytes_16(),
      random_bytes_16(),
      random_bytes_16(),
      random_bytes_16(),
    ];

    let plaintexts = [
      random_bytes_16(),
      random_bytes_16(),
      random_bytes_16(),
      random_bytes_16(),
      random_bytes_16(),
      random_bytes_16(),
      random_bytes_16(),
      random_bytes_16(),
    ];

    let ciphertext = unsafe { encode(keys[0], plaintexts[0], 66000) }; 

    let mut group = c.benchmark_group("aes-ni-simple");

    group.bench_function(
      "encode", 
      |b| b.iter(
        || unsafe { encode(keys[0], plaintexts[0], 66000) }
      )
    );

    group.bench_function(
      "encode-memory", 
      |b| b.iter(
        || unsafe { encode_memory(keys[0], plaintexts[0], 66000) }
      )
    );

    group.bench_function(
      "encode-with-keys", 
      |b| b.iter(
        || unsafe { encode_with_keys(keys, plaintexts[0], 66000) }
      )
    );

    group.bench_function(
      "decode", 
      |b| b.iter(
        || unsafe { decode(keys[0], ciphertext, 66000) }
      )
    );

    group.bench_function(
      "encode-pipelined", 
      |b| b.iter(
        || unsafe { encode_pipelined(keys[0], plaintexts, 66000) }
      )
    );

    group.bench_function(
      "encode-pipelined-with-keys", 
      |b| b.iter(
        || unsafe { encode_pipelined_with_keys(keys, plaintexts, 66000) }
      )
    );

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);