#[macro_use]
extern crate criterion;

use criterion::Criterion;
use flit::BloomFilter;
use rand::distributions::{Distribution, Standard};
use rand::thread_rng;
use std::hash::Hash;

fn add_to_filter<T: Hash>(filter: &mut BloomFilter<T>, items: &[T]) {
    for item in items {
        filter.add(&item);
    }
}

fn benchmark_add(c: &mut Criterion) {
    c.bench_function("add 100", |b| {
        let nums = get_random_nums(100);
        let mut filter = BloomFilter::new(0.01, 100);

        b.iter(|| add_to_filter(&mut filter, &nums));
    });

    c.bench_function("add 1000", |b| {
        let nums = get_random_nums(1000);
        let mut filter = BloomFilter::new(0.01, 1000);

        b.iter(|| add_to_filter(&mut filter, &nums));
    });

    c.bench_function("add 10000", |b| {
        let nums = get_random_nums(10000);
        let mut filter = BloomFilter::new(0.01, 10000);

        b.iter(|| add_to_filter(&mut filter, &nums));
    });
}

fn get_random_nums(n: usize) -> Vec<u32> {
    let mut rng = thread_rng();
    Standard.sample_iter(&mut rng).take(n).collect()
}

criterion_group!(benches, benchmark_add);
criterion_main!(benches);
