use super::naive_datetime_iter::NaiveDatetimeIterError;
use super::naive_datetime_iter::NaiveDatetimeIterator;
use chrono::{Duration, NaiveDateTime};

/// An iterator that produces consecutive datetime ranges
///
/// This iterator yields tuples of `(start, end)` datetimes where each range
/// represents a time period between consecutive steps.
///
/// # Examples
///
/// Forward iteration:
/// ```
/// use chrono_kit::iter::NaiveDatetimeRangeIterator;
/// use chrono::{NaiveDateTime, Duration};
///
/// let start = NaiveDateTime::parse_from_str("2023-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
/// let end = NaiveDateTime::parse_from_str("2023-01-03 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
/// let step = Duration::days(1);
///
/// let mut iter = NaiveDatetimeRangeIterator::new(start, end, step).unwrap();
/// assert_eq!(iter.next(), Some((start, start + step)));
/// assert_eq!(iter.next(), Some((start + step, end)));
/// ```
///
/// Reverse iteration:
/// ```
/// use chrono_kit::iter::NaiveDatetimeRangeIterator;
/// use chrono::{NaiveDateTime, Duration};
///
/// let start = NaiveDateTime::parse_from_str("2023-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
/// let end = NaiveDateTime::parse_from_str("2023-01-03 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
/// let step = Duration::days(-1);
///
/// let mut iter = NaiveDatetimeRangeIterator::new(start, end, step).unwrap();
/// assert_eq!(iter.next(), Some((end + step, end)));
/// assert_eq!(iter.next(), Some((start, end + step)));
/// ```
pub struct NaiveDatetimeRangeIterator {
    datetime_iter: NaiveDatetimeIterator,
    current: Option<NaiveDateTime>,
    asc: bool,
}

impl NaiveDatetimeRangeIterator {
    /// Creates a new DatetimeRangeIterator
    ///
    /// # Arguments
    /// * `start` - The starting datetime (inclusive)
    /// * `end` - The ending datetime (inclusive)
    /// * `step` - The duration between each step (must be non-zero)
    ///
    /// # Errors
    /// Returns `DatetimeIterError` if:
    /// - `step` is zero
    /// - `start` and `end` don't form a valid range for the given step
    pub fn new(
        start: NaiveDateTime,
        end: NaiveDateTime,
        step: Duration,
    ) -> Result<Self, NaiveDatetimeIterError> {
        let datetime_iter = NaiveDatetimeIterator::new(start, end, step)?;

        Ok(NaiveDatetimeRangeIterator {
            datetime_iter,
            current: None,
            asc: step > Duration::zero(),
        })
    }
}

impl Iterator for NaiveDatetimeRangeIterator {
    type Item = (NaiveDateTime, NaiveDateTime);

    fn next(&mut self) -> Option<Self::Item> {
        let start = match self.current {
            Some(dt) => dt,
            None => {
                let first = self.datetime_iter.next()?;
                self.current = Some(first);
                first
            }
        };

        let end = self.datetime_iter.next()?;
        self.current = Some(end);
        if self.asc {
            Some((start, end))
        } else {
            Some((end, start))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;

    #[test]
    fn test_ascending_range_iteration() {
        let start =
            NaiveDateTime::parse_from_str("2023-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let end =
            NaiveDateTime::parse_from_str("2023-01-03 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let step = Duration::days(1);

        let mut iter = NaiveDatetimeRangeIterator::new(start, end, step).unwrap();
        assert_eq!(iter.next(), Some((start, start + step)));
        assert_eq!(iter.next(), Some((start + step, end)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_descending_range_iteration() {
        let start =
            NaiveDateTime::parse_from_str("2023-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let end =
            NaiveDateTime::parse_from_str("2023-01-03 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let step = Duration::days(-1);

        let mut iter = NaiveDatetimeRangeIterator::new(start, end, step).unwrap();
        assert_eq!(iter.next(), Some((end + step, end)));
        assert_eq!(iter.next(), Some((start, end + step)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_non_integer_period() {
        let start =
            NaiveDateTime::parse_from_str("2023-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let end =
            NaiveDateTime::parse_from_str("2023-01-01 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let step = Duration::hours(9);

        let mut iter = NaiveDatetimeRangeIterator::new(start, end, step).unwrap();
        assert_eq!(iter.next(), Some((start, start + step)));
        assert_eq!(iter.next(), Some((start + step, end)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_single_range() {
        let start =
            NaiveDateTime::parse_from_str("2023-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let end =
            NaiveDateTime::parse_from_str("2023-01-01 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let step = Duration::hours(12);

        let mut iter = NaiveDatetimeRangeIterator::new(start, end, step).unwrap();
        assert_eq!(iter.next(), Some((start, end)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_zero_step_error() {
        let start =
            NaiveDateTime::parse_from_str("2023-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let end =
            NaiveDateTime::parse_from_str("2023-01-03 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let step = Duration::zero();

        let result = NaiveDatetimeRangeIterator::new(start, end, step);
        assert!(matches!(result, Err(NaiveDatetimeIterError::ZeroStep)));
    }

    #[test]
    fn test_invalid_range_error() {
        let start =
            NaiveDateTime::parse_from_str("2023-01-03 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let end =
            NaiveDateTime::parse_from_str("2023-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let step = Duration::days(1);

        let result = NaiveDatetimeRangeIterator::new(start, end, step);
        assert!(matches!(
            result,
            Err(NaiveDatetimeIterError::InvalidRange { .. })
        ));
    }
}
