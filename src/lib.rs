//! Time's hard. Correct time is near impossible.
//!
//! This crate has one purpose: give us a timestamp as an integer, coming from a monotonic clock
//! source, include time across suspend/hibernation of the host machine and let me compare it to
//! other timestamps.
//!
//! [`std::time::Instant`] gives us some of that:
//!
//! 1. It's monotonic, guaranteed.
//! 2. It can be compared to other timespans.
//!
//! However it can't be serialized and its not clear if it contains suspend/hibernation time across
//! different operating systems.
//!
//! The user can choose if their `zeitstempel::Instant` uses a clock that observes suspended time or not.
//! The resulting instant can be serialized into an integer.
//! It's the developer's responsibility to only compare timestamps from the same clocksource.
//! Timestamps are not comparable across operating system reboots.
//!
//! # Example
//!
//! ```
//! # use std::{thread, time::Duration};
//! use zeitstempel::Instant;
//!
//! let now = Instant::now_including_suspend();
//! thread::sleep(Duration::from_millis(2));
//!
//! let diff = now.elapsed();
//! assert!(diff >= Duration::from_millis(2));
//! ```

#![deny(missing_docs)]
#![deny(broken_intra_doc_links)]

use std::ops::Sub;
use std::time::Duration;

cfg_if::cfg_if! {
    if #[cfg(any(target_os = "macos", target_os = "ios"))] {
        mod mac;
        use mac as sys;
    } else if #[cfg(any(target_os = "linux", target_os = "android"))] {
        mod linux;
        use linux as sys;
    } else if #[cfg(windows)] {
        mod windows;
        use windows as sys;
    } else {
        mod unsupported;
        use unsupported as sys;
    }
}

/// Marker for an instant that comes from a clock source observing suspended time.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IncludingSuspend;

/// Marker for an instant that comes from a clock source *not* observing suspended time.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExcludingSuspend;

fn now_including_suspend() -> u64 {
    sys::now_including_suspend()
}

fn now_excluding_suspend() -> u64 {
    sys::now_excluding_suspend()
}

/// A measurement of a monotonically nondecreasing clock.
///
/// Depending on the clock source the time measured can observe suspended time or not observe it.
/// See [`Instant::now_including_suspend`] and [`Instant::now_excluding_suspend`].
///
/// Contains a timestamp represented as an integer, which can be serialized or stored.
/// Two `Instants` can be compared.
/// Their difference is the duration between their two points in time.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instant<T>(u64, T);

impl<T> Instant<T> {
    /// Returns the amount of time elapsed from another instant to this one,
    /// or None if that instant is later than this one.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::time::Duration;
    /// use std::thread::sleep;
    /// use zeitstempel::Instant;
    ///
    /// let now = Instant::now_including_suspend();
    /// sleep(Duration::new(1, 0));
    /// let new_now = Instant::now_including_suspend();
    /// println!("{:?}", new_now.checked_duration_since(now));
    /// println!("{:?}", now.checked_duration_since(new_now)); // None
    /// ```
    pub fn checked_duration_since(&self, earlier: Instant<T>) -> Option<Duration> {
        self.0.checked_sub(earlier.0).map(Duration::from_nanos)
    }

    /// Returns the amount of time elapsed from another instant to this one.
    ///
    /// # Panics
    ///
    /// This function will panic if `earlier` is later than `self`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::time::Duration;
    /// use std::thread::sleep;
    /// use zeitstempel::Instant;
    ///
    /// let now = Instant::now_including_suspend();
    /// sleep(Duration::new(1, 0));
    /// let new_now = Instant::now_including_suspend();
    /// println!("{:?}", new_now.duration_since(now));
    /// ```
    pub fn duration_since(&self, earlier: Instant<T>) -> Duration {
        self.checked_duration_since(earlier)
            .expect("supplied instant is later than self")
    }

    /// Get the underlying timestamp.
    pub fn as_timestamp(&self) -> u64 {
        self.0
    }
}

impl Instant<IncludingSuspend> {
    /// Returns an instant corresponding to "now", including suspended time.
    pub fn now_including_suspend() -> Self {
        Self::now()
    }

    fn now() -> Self {
        Instant(now_including_suspend(), IncludingSuspend)
    }

    /// Returns the amount of time elapsed since this instant was created.
    ///
    /// # Panics
    ///
    /// This function may panic if the current time is earlier than this
    /// instant, which is something that can happen if an `Instant` is
    /// produced synthetically.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::thread::sleep;
    /// use std::time::Duration;
    /// use zeitstempel::Instant;
    ///
    /// let instant = Instant::now_including_suspend();
    /// let three_secs = Duration::from_secs(3);
    /// sleep(three_secs);
    /// assert!(instant.elapsed() >= three_secs);
    /// ```
    pub fn elapsed(&self) -> Duration {
        Self::now() - *self
    }
}

impl Instant<ExcludingSuspend> {
    /// Returns an instant corresponding to "now", excluding suspended time.
    pub fn now_excluding_suspend() -> Self {
        Self::now()
    }

    fn now() -> Self {
        Instant(now_excluding_suspend(), ExcludingSuspend)
    }

    /// Returns the amount of time elapsed since this instant was created.
    ///
    /// # Panics
    ///
    /// This function may panic if the current time is earlier than this
    /// instant, which is something that can happen if an `Instant` is
    /// produced synthetically.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::thread::sleep;
    /// use std::time::Duration;
    /// use zeitstempel::Instant;
    ///
    /// let instant = Instant::now_excluding_suspend();
    /// let three_secs = Duration::from_secs(3);
    /// sleep(three_secs);
    /// assert!(instant.elapsed() >= three_secs);
    /// ```
    pub fn elapsed(&self) -> Duration {
        Self::now() - *self
    }
}

impl<T> Sub<Instant<T>> for Instant<T> {
    type Output = Duration;

    fn sub(self, other: Instant<T>) -> Duration {
        self.duration_since(other)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::thread;

    #[test]
    fn order() {
        let ts1 = Instant::now_including_suspend();
        let ts2 = Instant::now_including_suspend();

        assert!(ts1 <= ts2);
        assert!(ts2 >= ts1);

        let ts1 = Instant::now_excluding_suspend();
        let ts2 = Instant::now_excluding_suspend();

        assert!(ts1 <= ts2);
        assert!(ts2 >= ts1);
    }

    #[test]
    fn times_are_close_together() {
        let ts_with = Instant::now_including_suspend();
        let ts_wo = Instant::now_excluding_suspend();

        thread::sleep(Duration::from_millis(2));

        let diff_with = ts_with.elapsed().as_millis() as i64;
        let diff_wo = ts_wo.elapsed().as_millis() as i64;

        let diff = (diff_with - diff_wo).abs();

        let max_delta_ms = 10;
        assert!(
            diff < max_delta_ms,
            "In test condition, the two uptimes should be close to each other"
        );
    }
}
