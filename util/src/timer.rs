//! A timer module containing a timer that counts a specific amount of time. Uses the hibernation
//! clock to count time.

use core::time::Duration;
use tm4c123x_hal::tm4c123x::HIB;

/// The timer struct. Used to count a specific amount of time with the hibernation clock. Timers
/// will only work properly if the uptime of the system is less than 2^32 seconds (~136.2 years)
/// at the time of timer polling. Timers have an accuracy of 1/32768 seconds.
pub struct Timer<'a> {
    hib: &'a HIB,
    end_subseconds: u64,
}

impl<'a> Timer<'a> {
    const SUBSECONDS_PER_SECOND: u64 = 32_768;
    const MICROSECONDS_PER_SECOND: u64 = 1_000_000;

    /// Converts (seconds, subseconds) to subseconds.
    fn time_to_subseconds((sec, subsec): (u32, u16)) -> u64 {
        return (sec as u64) * Self::SUBSECONDS_PER_SECOND + (subsec as u64);
    }

    /// Gets the current time from the hibernation clock.
    fn get_time_hib(hib: &HIB) -> (u32, u16) {
        loop {
            // A read from the RTC is only valid when the seconds count is the same before and after
            // retrieving the subseconds count.
            let seconds = hib.rtcc.read().bits();
            let subsec = hib.rtcss.read().rtcssc().bits();

            if seconds == hib.rtcc.read().bits() {
                return (seconds, subsec);
            }
        }
    }

    /// Gets the current time from the hibernation clock.
    fn get_time(&self) -> (u32, u16) {
        Self::get_time_hib(self.hib)
    }

    fn new_impl(hib: &'a HIB, duration: Duration) -> Self {
        let curr_subseconds = Self::time_to_subseconds(Self::get_time_hib(hib));

        let duration_secs = duration
            .as_secs()
            .try_into()
            .expect("Duration is too long.");

        let duration_subsecs = (duration.subsec_micros() as u64 * Self::SUBSECONDS_PER_SECOND
            / Self::MICROSECONDS_PER_SECOND) as u16;

        let subsecond_duration = Self::time_to_subseconds((duration_secs, duration_subsecs));

        Timer {
            hib,
            end_subseconds: curr_subseconds + subsecond_duration,
        }
    }

    #[cfg(not(debug_assertions))]
    /// Initializes a timer that expires after a certain duration.
    pub fn new(hib: &'a HIB, duration: Duration) -> Self {
        Self::new_impl(hib, duration)
    }

    #[cfg(debug_assertions)]
    /// Initializes a timer that expires after a certain duration.
    pub fn new(hib: &'a HIB, duration: Duration) -> Self {
        Self::new_impl(hib, duration)
    }

    /// Polls a timer. Returns whether the timer has expired.
    pub fn poll(&self) -> bool {
        Self::time_to_subseconds(self.get_time()) >= self.end_subseconds
    }
}
