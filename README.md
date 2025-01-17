[![crate](https://img.shields.io/crates/v/iter_index.svg)](https://crates.io/crates/iter-index)
[![documentation](https://docs.rs/index-start/badge.svg)](https://docs.rs/iter-index)
[![crate](https://img.shields.io/crates/l/iter_index.svg)](https://github.com/blueglyph/iter_index/blob/master/LICENSE-MIT)
[![crate](https://img.shields.io/crates/l/iter_index.svg)](https://github.com/blueglyph/iter_index/blob/master/LICENSE-APACHE)

# `iter-index` crate

This is a simple extension trait that provides a more flexible alternative to the iterator's method `enumerate()`. It allows to:
 * use a custom type for the index with `index::<T>()`
 * define a custom start value with `index_start::<T>(start: T)`
 * define a custom step value with `index_step::<T>(start: T, step: T)`.

## Examples

```rust
use iter_index::IndexerIterator;

let items = vec!["a", "b", "c"];
let mut result = items.into_iter().index::<i32>();

assert_eq!(result.next(), Some((0_i32, "a")));
assert_eq!(result.next(), Some((1_i32, "b")));
assert_eq!(result.next(), Some((2_i32, "c")));
assert_eq!(result.next(), None);
```

```rust
let items = vec!["a", "b", "c"];
let mut result = items.into_iter().index_start::<u8>(97);

assert_eq!(result.next(), Some((97_u8, "a")));
assert_eq!(result.next(), Some((98_u8, "b")));
assert_eq!(result.next(), Some((99_u8, "c")));
assert_eq!(result.next(), None);
```

```rust
let items = vec!["a", "b", "c"];
let mut result = items.into_iter().index_step::<u32>(100, 10);

assert_eq!(result.next(), Some((100_u32, "a")));
assert_eq!(result.next(), Some((110_u32, "b")));
assert_eq!(result.next(), Some((120_u32, "c")));
assert_eq!(result.next(), None);
```

# License

This code is licensed under either [MIT License](https://choosealicense.com/licenses/mit/) or [Apache License 2.0](https://choosealicense.com/licenses/apache-2.0/).
