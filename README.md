# Chrono-kit

<!-- These badges will become active after publishing to crates.io and setting up CI -->
[![Crates.io](https://img.shields.io/crates/v/chrono-kit)](https://crates.io/crates/chrono-kit)
[![Documentation](https://docs.rs/chrono-kit/badge.svg)](https://docs.rs/chrono-kit)
[![License](https://img.shields.io/crates/l/chrono-kit)](LICENSE)
[![CI Status](https://github.com/wangLiu-gh/chrono-kit/workflows/Rust/badge.svg)](https://github.com/wangLiu-gh/chrono-kit/actions)
[![Coverage](https://codecov.io/gh/wangLiu-gh/chrono-kit/branch/main/graph/badge.svg)](https://codecov.io/gh/wangLiu-gh/chrono-kit)

A time manipulation toolkit built on chrono, providing convenient iterators and utilities for working with dates and times.

## Features

Currently implemented features:
- NaiveDateTime range iteration (`NaiveDatetimeRangeIterator`)
- NaiveDateTime iteration (`NaiveDatetimeIterator`)
- Basic time range calculations

Want to see something added? Open an issue with your feature request!

## Usage

Add to your `Cargo.toml`:
```toml
[dependencies]
chrono-kit = "0.1"
```

## Examples

```rust
use chrono_kit::iter::NaiveDatetimeRangeIterator;
use chrono::{NaiveDateTime, Duration};

let start = NaiveDateTime::parse_from_str("2023-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
let end = NaiveDateTime::parse_from_str("2023-01-03 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
let step = Duration::hours(1);

let mut iter = NaiveDatetimeRangeIterator::new(start, end, step).unwrap();
for (range_start, range_end) in iter {
    println!("Range: {} to {}", range_start, range_end);
}
```

## License

Dual-licensed under MIT or Apache 2.0 at your option.
