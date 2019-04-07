//! This crate provides a couple of simple varieties of Bloom filters:
//!
//! - [`BloomFilter`] is a standard Bloom filter implementation. Items can be added to the filter,
//! but cannot be removed. It is a very space-efficient data structure.
//! - `CountingBloomFilter` (not yet implemented) is a Counting Bloom filter implementation. Items can both be added
//! and removed. The trade off is that it has much higher space requirements than a standard Bloom
//! filter.
//!
//! [`BloomFilter`]: bloom_filter/struct.BloomFilter.html
pub mod bloom_filter;
pub use bloom_filter::BloomFilter;
