use std::f64::consts::LN_2;
use std::hash::Hash;

const LN2_SQUARED: f64 = LN_2 * LN_2;

#[derive(Debug)]
pub struct BloomFilter {
    n: usize,
    m: usize,
    k: usize,
}

impl BloomFilter {
    pub fn new(false_positive_rate: f64, estimated_items: usize) -> Self {
        assert!(false_positive_rate > 0_f64 && false_positive_rate < 1_f64);
        assert!(estimated_items > 0);

        let num_bits = -(estimated_items as f64) * false_positive_rate.ln() / LN2_SQUARED;
        let num_hashes = (num_bits / estimated_items as f64) * LN_2;

        BloomFilter {
            n: 0,
            m: num_bits.ceil() as usize,
            k: num_hashes.ceil() as usize,
        }
    }

    pub fn add<T: Hash>(item: T) {}

    pub fn false_positive_rate(&self) -> f64 {
        (1_f64 - std::f64::consts::E.powf(-1_f64 * self.k as f64 * self.n as f64 / self.m as f64))
            .powf(self.k as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let filter = BloomFilter::new(0.01_f64, 216553);
        dbg!(&filter);
        dbg!(&filter.false_positive_rate());
    }
}
