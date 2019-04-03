# flit

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

