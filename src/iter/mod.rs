//! Iterators for datetime ranges
//!
//! This module provides iterators for working with datetime ranges:
//! - `NaiveDatetimeIterator`: Iterates through individual datetimes
//! - `NaiveDatetimeRangeIterator`: Iterates through datetime ranges
//!
//! Both iterators support forward and reverse iteration:
//! - Forward iteration: Use positive step duration
//! - Reverse iteration: Use negative step duration
//!
//! See the individual iterator documentation for examples.

mod naive_datetime_iter;
mod naive_datetime_range_iter;

pub use naive_datetime_iter::*;
pub use naive_datetime_range_iter::*;
