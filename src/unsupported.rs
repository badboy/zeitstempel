use std::convert::TryInto;
use std::time::Instant;

use once_cell::sync::Lazy;

static INIT_TIME: Lazy<Instant> = Lazy::new(Instant::now);

pub fn now_including_suspend_ms() -> u64 {
    let d = INIT_TIME.elapsed();
    d.as_millis().try_into().unwrap_or(0)
}

pub fn now_excluding_suspend_ms() -> u64 {
    let d = INIT_TIME.elapsed();
    d.as_millis().try_into().unwrap_or(0)
}
