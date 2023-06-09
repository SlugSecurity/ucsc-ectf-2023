#![cfg(debug_assertions)]

use core::time::Duration;
use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use tm4c123x_hal::delay::Delay;
use ucsc_ectf_util_no_std::{
    timer::{HibTimer, Timer},
    Arc, HibPool,
};

pub fn run(hib: &Arc<HibPool>, delay: &mut Delay) {
    too_slow_ms_test(hib, delay);
    too_fast_ms_test(hib, delay);
    too_slow_ms_repeated_test(hib, delay);
    too_fast_ms_repeated_test(hib, delay);
    immediate_return(hib, delay);
}

/// Tests if the timer is done 1 ms after the duration of the timer.
fn too_slow_ms_test(hib: &Arc<HibPool>, delay: &mut Delay) {
    let mut new_timer = HibTimer::new(hib, Duration::from_millis(629));

    delay.delay_ms(630u32);

    assert!(new_timer.poll())
}

/// Tests if the timer isn't done 1 ms before the duration of the timer.
fn too_fast_ms_test(hib: &Arc<HibPool>, delay: &mut Delay) {
    let mut new_timer = HibTimer::new(hib, Duration::from_millis(548));

    delay.delay_ms(547u32);

    assert!(!new_timer.poll())
}

/// Tests if the timer is done 1 ms after the duration of the timer 1000 times.
fn too_slow_ms_repeated_test(hib: &Arc<HibPool>, delay: &mut Delay) {
    for _ in 0..1000 {
        let mut new_timer = HibTimer::new(hib, Duration::from_millis(1));

        delay.delay_ms(2u32);

        assert!(new_timer.poll());
    }
}

/// Tests if the timer isn't done 1 ms before the duration of the timer 1000 times.
fn too_fast_ms_repeated_test(hib: &Arc<HibPool>, delay: &mut Delay) {
    for _ in 0..1000 {
        let mut new_timer = HibTimer::new(hib, Duration::from_millis(2));

        delay.delay_ms(1u32);

        assert!(!new_timer.poll());
    }
}

/// Checks if a timer with a duration of 0 expires immediately.
fn immediate_return(hib: &Arc<HibPool>, _delay: &mut Delay) {
    let mut new_timer = HibTimer::new(hib, Duration::from_secs(0));

    assert!(new_timer.poll())
}
