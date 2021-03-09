use winapi::um::winnt::PULONGLONG;

#[link(name = "onecoreuap")]
extern "system" {
    fn QueryInterruptTime(InterruptTime: PULONGLONG);
}

/// Windows counts time in a system time unit of 100 nanoseconds.
const SYSTEM_TIME_UNIT: u64 = 100;

/// The time based on the current interrupt-time count.
/// This includes the suspend time.
///
/// See [`QueryInterruptTime`].
///
/// [`QueryInterruptTime`]: https://docs.microsoft.com/en-us/windows/win32/api/realtimeapiset/nf-realtimeapiset-queryinterrupttime
pub fn now_including_suspend() -> u64 {
    let mut interrupt_time = 0;
    unsafe {
        QueryInterruptTime(&mut interrupt_time);
    }

    interrupt_time * SYSTEM_TIME_UNIT
}
