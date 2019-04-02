use std::f64::consts::LN_2;
use std::hash::Hash;

const LN2_SQUARED: f64 = LN_2 * LN_2;

/// `BloomFilter` is a probabilistic data structure that can definitively conclude that it does
/// *not* contain an item. On the other hand, it can only conclude that it *probably* contains an
/// item.
///
/// Items can be added to the Bloom filter, but cannot be removed - this would introduce false
/// negative cases. Once added, the structure can be quickly queried for the existence of the item.
///
/// Some good references include:
/// - [Wikipedia article](https://en.wikipedia.org/wiki/Bloom_filter)
/// - [Bloom Filters by Example](https://llimllib.github.io/bloomfilter-tutorial/)
/// - [Bloom Filter Calculator](https://hur.st/bloomfilter/)
#[derive(Debug)]
pub struct BloomFilter {
    n: usize,
    m: usize,
    k: usize,
}

impl BloomFilter {
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

        BloomFilter {
            n: 0,
            m: num_bits.ceil() as usize,
            k: num_hashes.ceil() as usize,
        }
    }

    pub fn add<T: Hash>(item: T) {}

    /// Calculates the current expected false positive rate given the number of items in the
    /// filter.
    pub fn false_positive_rate(&self) -> f64 {
        (1_f64 - std::f64::consts::E.powf(-1_f64 * self.k as f64 * self.n as f64 / self.m as f64))
            .powi(self.k as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_bits_and_hashes() {
        let filter = BloomFilter::new(0.01_f64, 216553);

        assert_eq!(filter.m, 2_075_674);
        assert_eq!(filter.k, 7);
    }

    #[test]
    fn test_false_positive_rate_empty() {
        let filter = BloomFilter::new(0.01_f64, 216553);

        // False positive rate with nothing added to the filter should be 0.
        assert_eq!(filter.false_positive_rate(), 0_f64);
    }
}
