# flit

[![](http://meritbadge.herokuapp.com/flit)](https://crates.io/crates/flit)
[![](https://img.shields.io/badge/docs-flit%20v0.1.2-green.svg)](https://docs.rs/flit/0.1.2/flit/)
![](https://github.com/tverghis/flit/workflows/Rust/badge.svg?branch=master)

`BloomFilter` is a probabilistic data structure that can quickly and definitively conclude that it does
*not* contain an item. On the other hand, it can only conclude that it *probably* contains an
item, i.e., the data structure has an inherent false-positive rate greater than 0%.

Items can be added to the Bloom filter, but cannot be removed - this would introduce false
negative cases. If this is required, an alternative might be to use a Counting Bloom filter
(not (yet) implemented in this crate).

This implementation is backed by a Rust implementation of the [xxHash hashing algorithm](https://github.com/Cyan4973/xxHash), [twox-hash](https://crates.io/crates/twox-hash).

## References
- [Less Hashing, Same Performance: Building a Better Bloom Filter](https://www.eecs.harvard.edu/~michaelm/postscripts/rsa2008.pdf)
- [Wikipedia article](https://en.wikipedia.org/wiki/Bloom_filter)
- [Bloom Filters by Example](https://llimllib.github.io/bloomfilter-tutorial/)
- [Bloom Filter Calculator](https://hur.st/bloomfilter/)
 
## Example
```rust
use flit::BloomFilter;

let mut filter = BloomFilter::new(0.01, 10000);
filter.add(&"Hello, world!");

assert_eq!(filter.might_contain(&"Hello, world!"), true); // probably true
assert_eq!(filter.might_contain(&"Dogs are cool!"), false); // definitely false!
```

## Benchmarks

Benchmarking is done using [`criterion`](https://crates.io/crates/criterion).

Simple benchmarks are provided for adding 100, 1000 and 10000 `u32`s to a BloomFilter. To run the benchmarks, run:
```
cargo bench

add 100                 time:   [22.454 us 22.477 us 22.507 us]
add 1000                time:   [224.44 us 224.65 us 224.90 us]
add 10000               time:   [2.2424 ms 2.2443 ms 2.2463 ms]
```
