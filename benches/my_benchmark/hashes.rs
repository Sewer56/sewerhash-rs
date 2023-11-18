use ahash::RandomState;
use criterion::black_box;
use criterion::Criterion;
use sewerhash_rs::implementations::binary::sewerhash_nonvector_64::unstable_hash_non_vector64;

pub(crate) fn hash_benchmark_nonvector_64(c: &mut Criterion) {
    let data_sizes = [6, 8, 10, 12, 16, 32, 64, 128, 256, 512, 1024];

    for &size in &data_sizes {
        let data = vec![0u8; size];
        c.bench_function(&format!("[sh64] hash {} bytes", size), |b| {
            b.iter(|| unstable_hash_non_vector64(black_box(&data)))
        });

        c.bench_function(&format!("[fnv-64] hash {} bytes", size), |b| {
            b.iter(|| fnv_hash(black_box(&data)))
        });

        c.bench_function(&format!("[ahash] hash {} bytes", size), |b| {
            b.iter(|| {
                let build_hasher = RandomState::with_seeds(1, 2, 3, 4);
                build_hasher.hash_one(black_box(&data))
            })
        });
    }
}

// Exported from FNV crate.
const INITIAL_STATE: u64 = 0xcbf29ce484222325;
const PRIME: u64 = 0x100000001b3;
#[inline]
pub const fn fnv_hash(bytes: &[u8]) -> u64 {
    let mut hash = INITIAL_STATE;
    let mut i = 0;
    while i < bytes.len() {
        hash ^= bytes[i] as u64;
        hash = hash.wrapping_mul(PRIME);
        i += 1;
    }
    hash
}
