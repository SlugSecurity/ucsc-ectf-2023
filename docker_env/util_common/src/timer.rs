//! Provides a common trait to implement a timer with the [`Timer`] trait.

/// This trait represents a timer that can be poll and reset. It is used to provide a platform-indepentdent
/// way to poll for whether time is up and reset the timer.
pub trait Timer {
    /// Polls the timer to see if time is up, returning ``true`` if it is up.
    fn poll(&mut self) -> bool;

    /// Resets the timer back to its original duration.
    fn reset(&mut self);
}
