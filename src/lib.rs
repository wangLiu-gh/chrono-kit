//! A time manipulation toolkit built on chrono
//!
//! Provides convenient iterators and utilities for working with naive dates and times.
//!
//! # Examples
//!
//! ```
//! use chrono_kit::iter::NaiveDatetimeRangeIterator;
//! use chrono::{NaiveDateTime, Duration};
//!
//! let start = NaiveDateTime::parse_from_str("2023-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
//! let end = NaiveDateTime::parse_from_str("2023-01-03 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
//! let step = Duration::hours(1);
//!
//! let mut iter = NaiveDatetimeRangeIterator::new(start, end, step).unwrap();
//! for (range_start, range_end) in iter {
//!     println!("Range: {} to {}", range_start, range_end);
//! }
//! ```

pub mod iter;
