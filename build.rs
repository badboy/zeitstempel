#[cfg(windows)]
fn main() {
    if std::env::var("CARGO_CFG_WINDOWS").is_ok() {
        windows::build!(
            windows::win32::windows_programming::{QueryUnbiasedInterruptTime, QueryInterruptTime}
        );
    }
}

#[cfg(not(windows))]
fn main() {}
