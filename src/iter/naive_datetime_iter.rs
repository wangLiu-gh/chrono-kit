use chrono::{Duration, NaiveDateTime};
use thiserror::Error;

/// Errors that can occur when creating a datetime iterator
#[derive(Debug, Error)]
pub enum NaiveDatetimeIterError {
    /// Returned when step duration is zero
    #[error("Step duration cannot be zero")]
    ZeroStep,
    /// Returned when start datetime is after end datetime for positive step
    #[error("Invalid range: start {start} must be before end {end} for positive step")]
    InvalidRange {
        start: NaiveDateTime,
        end: NaiveDateTime,
    },
}

/// Iterator that yields datetimes between start and end with given step
///
/// Handles both ascending and descending iteration based on step sign.
pub struct NaiveDatetimeIterator {
    start: NaiveDateTime,
    end: NaiveDateTime,
    step: Duration,
}

impl NaiveDatetimeIterator {
    /// Creates a new DatetimeIterator
    ///
    /// # Arguments
    /// * `start` - The starting datetime (inclusive)
    /// * `end` - The ending datetime (inclusive)
    /// * `step` - The duration between each step (must be non-zero)
    ///
    /// # Errors
    /// Returns `DatetimeIterError` if:
    /// - `step` is zero
    /// - `start` is after `end` for positive step
    pub fn new(
        start: NaiveDateTime,
        end: NaiveDateTime,
        step: Duration,
    ) -> Result<Self, NaiveDatetimeIterError> {
        if step.is_zero() {
            return Err(NaiveDatetimeIterError::ZeroStep);
        }
        if start > end {
            return Err(NaiveDatetimeIterError::InvalidRange { start, end });
        }
        Ok(NaiveDatetimeIterator { start, end, step })
    }

    fn next_asc(&mut self) -> Option<NaiveDateTime> {
        if self.start > self.end {
            return None;
        }

        let result = self.start;
        self.start = if self.start < self.end && self.start + self.step > self.end {
            self.end
        } else {
            self.start + self.step
        };

        Some(result)
    }

    fn next_desc(&mut self) -> Option<NaiveDateTime> {
        if self.end < self.start {
            return None;
        }

        let result = self.end;
        self.end = if self.end > self.start && self.end + self.step < self.start {
            self.start
        } else {
            self.end + self.step
        };

        Some(result)
    }
}

impl Iterator for NaiveDatetimeIterator {
    type Item = NaiveDateTime;

    fn next(&mut self) -> Option<Self::Item> {
        if self.step > Duration::zero() {
            self.next_asc()
        } else {
            self.next_desc()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascending_iteration() {
        let start =
            NaiveDateTime::parse_from_str("2023-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let end =
            NaiveDateTime::parse_from_str("2023-01-03 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let step = Duration::days(1);

        let mut iter = NaiveDatetimeIterator::new(start, end, step).unwrap();
        assert_eq!(iter.next(), Some(start));
        assert_eq!(iter.next(), Some(start + step));
        assert_eq!(iter.next(), Some(end));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_descending_iteration() {
        let start =
            NaiveDateTime::parse_from_str("2023-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let end =
            NaiveDateTime::parse_from_str("2023-01-03 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let step = Duration::days(-1);

        let mut iter = NaiveDatetimeIterator::new(start, end, step).unwrap();
        assert_eq!(iter.next(), Some(end));
        assert_eq!(iter.next(), Some(end + step));
        assert_eq!(iter.next(), Some(start));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_zero_step_error() {
        let start =
            NaiveDateTime::parse_from_str("2023-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let end =
            NaiveDateTime::parse_from_str("2023-01-03 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let step = Duration::zero();

        let result = NaiveDatetimeIterator::new(start, end, step);
        assert!(matches!(result, Err(NaiveDatetimeIterError::ZeroStep)));
    }

    #[test]
    fn test_invalid_range_error() {
        let start =
            NaiveDateTime::parse_from_str("2023-01-03 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let end =
            NaiveDateTime::parse_from_str("2023-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let step = Duration::days(1);

        let result = NaiveDatetimeIterator::new(start, end, step);
        assert!(matches!(
            result,
            Err(NaiveDatetimeIterError::InvalidRange { .. })
        ));
    }

    #[test]
    fn test_non_integer_step() {
        let start =
            NaiveDateTime::parse_from_str("2023-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let end =
            NaiveDateTime::parse_from_str("2023-01-03 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let step = Duration::days(1);

        let mut iter = NaiveDatetimeIterator::new(start, end, step).unwrap();
        assert_eq!(iter.next(), Some(start));
        assert_eq!(iter.next(), Some(start + step));
        assert_eq!(iter.next(), Some(start + step * 2));
        assert_eq!(iter.next(), Some(end));
    }
}
