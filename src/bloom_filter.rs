//! `BloomFilter` is a probabilistic data structure that can quickly and definitively conclude that it does
//! *not* contain an item. On the other hand, it can only conclude that it *probably* contains an
//! item, i.e., the data structure has an inherent false-positive rate greater than 0%.
//!
//! Items can be added to the Bloom filter, but cannot be removed - this would introduce false
//! negative cases. If this is required, an alternative might be to use a Counting Bloom filter
//! (not (yet) implemented in this crate).
//!
//! This allows the filter to be very space-efficient.
//!
//! This implementation is backed by a Rust implementation of the [xxHash hashing
//! algorithm](https://github.com/Cyan4973/xxHash), [twox-hash](https://crates.io/crates/twox-hash).
//!
//! # References
//! - [Less Hashing, Same Performance: Building a Better Bloom
//! Filter](https://www.eecs.harvard.edu/~michaelm/postscripts/rsa2008.pdf)
//! - [Wikipedia article](https://en.wikipedia.org/wiki/Bloom_filter)
//! - [Bloom Filters by Example](https://llimllib.github.io/bloomfilter-tutorial/)
//! - [Bloom Filter Calculator](https://hur.st/bloomfilter/)

use bitvec::{bitvec, BitVec};
use std::f64::consts::{E, LN_2};
use std::hash::{BuildHasher, Hash, Hasher};
use std::marker::PhantomData;
use twox_hash::RandomXxHashBuilder;

const LN2_SQUARED: f64 = LN_2 * LN_2;

/// Represents a Bloom filter.
///
/// When constructing the filter using `new`, you need to specify the desired acceptable
/// false-positive rate, and the number of items you intend to store in the filter. Neither can be
/// adjusted after creation - create a new filter instead.
///
/// # Example
/// ```rust
/// use flit::BloomFilter;
///
/// let mut filter = BloomFilter::new(0.01, 10000);
/// filter.add(&"Hello, world!");
///
/// assert_eq!(filter.might_contain(&"Hello, world!"), true); // probably true
/// assert_eq!(filter.might_contain(&"Dogs are cool!"), false); // definitely false!
/// ```
pub struct BloomFilter<T> {
    n: u64,
    m: u64,
    k: u32,
    bit_vec: BitVec,
    build_hasher: RandomXxHashBuilder,
    _phantom: PhantomData<T>,
}

impl<T: Hash> BloomFilter<T> {
    /// Creates a new Bloom filter based on the required false positive rate and the estimated
    /// number of items that will be added to the filter.
    ///
    /// The parameters influence the size of the filter, as well as the number of
    /// hashes that must be applied to the items.
    ///
    /// # Panics
    ///
    /// This function will panic if `false_positive_rate` is not between 0 and 1 (non inclusive),
    /// or if `estimated_items` is not greater than 0.
    pub fn new(false_positive_rate: f64, estimated_items: usize) -> Self {
        assert!(
            false_positive_rate > 0_f64 && false_positive_rate < 1_f64,
            "False positive rate must be between 0 and 1 (non-inclusive)"
        );
        assert!(
            estimated_items > 0,
            "Number of estimated items must be greater than zero"
        );

        let num_bits = -(estimated_items as f64) * false_positive_rate.ln() / LN2_SQUARED;
        let num_hashes = (num_bits / estimated_items as f64) * LN_2;

        let num_bits = num_bits.ceil() as u64;
        let num_hashes = num_hashes.ceil() as u32;

        BloomFilter {
            n: 0,
            m: num_bits,
            k: num_hashes,
            bit_vec: bitvec![0; num_bits as usize],
            build_hasher: RandomXxHashBuilder::default(),
            _phantom: PhantomData,
        }
    }

    /// Adds the `item` to the filter by setting the appropriate bits in the filter to `true`.
    pub fn add(&mut self, item: &T) {
        for i in indices_for_hash(split_hash(item, &self.build_hasher), self.m, self.k) {
            self.bit_vec.set(i, true);
        }

        self.n += 1;
    }

    /// Checks if the filter *might* contain the `item`.
    ///
    /// If this function returns false, the filter definitely does not contain the item.
    /// If this function returns true, the filter *might* contain the item, but it might also be a
    /// false-positive.
    pub fn might_contain(&self, item: &T) -> bool {
        for i in indices_for_hash(split_hash(item, &self.build_hasher), self.m, self.k) {
            if !self.bit_vec[i] {
                return false;
            }
        }

        true
    }

    /// Calculates the current expected false positive rate given the number of items in the
    /// filter.
    pub fn false_positive_rate(&self) -> f64 {
        (1_f64 - E.powf(-1_f64 * f64::from(self.k) * self.n as f64 / self.m as f64))
            .powi(self.k as i32)
    }
}

/// Hashes `item` using a `Hasher`, and produces a two-element tuple.
/// The first element is the "upper half" of the `u64` produced by the hash function, and the second
/// element is the "lower half".
fn split_hash<T: Hash>(item: &T, hasher: &impl BuildHasher) -> (u32, u32) {
    let mut hasher = hasher.build_hasher();
    item.hash(&mut hasher);
    let hash = hasher.finish();

    (((hash >> 32) as u32), hash as u32)
}

/// Returns the indices to be set to "true" in a Bloom filter for a given hash.
///
/// `split_hash` is a tuple of two `u32` values produced by passing the item to be added through
/// the `split_hash` function.
/// `m` is the number of indices in the filter.
/// `k` is the number of hash functions that the item should be passed through.
fn indices_for_hash(split_hash: (u32, u32), m: u64, k: u32) -> impl Iterator<Item = usize> {
    (0..k).map(move |i| {
        (u64::from(split_hash.0.wrapping_add(split_hash.1.wrapping_mul(i))) % m) as usize
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_bits_and_hashes() {
        let filter = BloomFilter::<&str>::new(0.01_f64, 216553);

        assert_eq!(filter.m, 2_075_674);
        assert_eq!(filter.k, 7);
    }

    #[test]
    fn test_false_positive_rate_empty() {
        let filter = BloomFilter::<&str>::new(0.01_f64, 216553);

        // False positive rate with nothing added to the filter should be 0.
        assert_eq!(filter.false_positive_rate(), 0_f64);
    }

    #[test]
    fn test_add() {
        let mut filter = BloomFilter::new(0.03_f64, 10);

        filter.add(&"Hello, world!");

        assert!(filter.false_positive_rate() > 0.0);
        assert_eq!(filter.might_contain(&"Hello, world!"), true);
        assert_eq!(filter.might_contain(&"Dogs are cool!"), false);
    }
}
