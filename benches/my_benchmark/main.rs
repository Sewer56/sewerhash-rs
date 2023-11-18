mod hashes;
use criterion::{criterion_group, criterion_main, Criterion};
use hashes::hash_benchmark_nonvector_64;

#[cfg(not(target_os = "windows"))]
use pprof::criterion::{Output, PProfProfiler};

fn criterion_benchmark(c: &mut Criterion) {
    hash_benchmark_nonvector_64(c);
}

#[cfg(not(target_os = "windows"))]
criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = criterion_benchmark
}

#[cfg(target_os = "windows")]
criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = criterion_benchmark
}

criterion_main!(benches);
