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
//! It's the developer's responsibility to only compare timestamps from the same clocksource.
//! Timestamps are not comparable across operating system reboots.
//!
//! # Example
//!
//! ```
//! # use std::{thread, time::Duration};
//!
//! let start = zeitstempel::now();
//! thread::sleep(Duration::from_millis(2));
//!
//! let diff = Duration::from_nanos(zeitstempel::now() - start);
//! assert!(diff >= Duration::from_millis(2));
//! ```

#![deny(missing_docs)]
#![deny(broken_intra_doc_links)]

cfg_if::cfg_if! {
    if #[cfg(any(target_os = "macos", target_os = "ios"))] {
        mod mac;
        use mac as sys;
    } else if #[cfg(any(target_os = "linux", target_os = "android"))] {
        mod linux;
        use linux as sys;
    } else if #[cfg(windows)] {
        mod win;
        use win as sys;
    } else {
        mod unsupported;
        use unsupported as sys;
    }
}

/// Returns a timestamp corresponding to "now".
///
/// It can be compared to other timestamps gathered from this API, as long as the host was not
/// rebooted inbetween.
///
///
/// ## Note
///
/// * The difference between two timestamps will include time the system was in sleep or
///   hibernation.
/// * The difference between two timestamps gathered from this is in nanoseconds.
/// * The clocks on some operating systems, e.g. on Windows, are not nanosecond-precise.
///   The value will still use nanosecond resolution.
pub fn now() -> u64 {
    sys::now_including_suspend()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn order() {
        let ts1 = now();
        thread::sleep(Duration::from_millis(2));
        let ts2 = now();

        assert!(ts1 < ts2);
    }
}
