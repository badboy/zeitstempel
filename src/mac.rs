#![allow(non_camel_case_types)]

use libc::clockid_t;

extern "C" {
    fn clock_gettime_nsec_np(clock_id: clockid_t) -> u64;
}

const CLOCK_MONOTONIC_RAW: clockid_t = 4;
const CLOCK_UPTIME_RAW: clockid_t = 8;

/// The time from a clock that increments monotonically,
/// but does not not increment while the system is asleep.
///
/// See [`clock_gettime_nsec_np`].
///
/// [`clock_gettime_nsec_np`]: https://opensource.apple.com/source/Libc/Libc-1158.1.2/gen/clock_gettime.3.auto.html
pub fn now_excluding_suspend() -> u64 {
    unsafe { clock_gettime_nsec_np(CLOCK_UPTIME_RAW) }
}

/// The time from a clock that increments monotonically,
/// tracking the time since an arbitrary point.
///
/// See [`clock_gettime_nsec_np`].
///
/// [`clock_gettime_nsec_np`]: https://opensource.apple.com/source/Libc/Libc-1158.1.2/gen/clock_gettime.3.auto.html
pub fn now_including_suspend() -> u64 {
    unsafe { clock_gettime_nsec_np(CLOCK_MONOTONIC_RAW) }
}
