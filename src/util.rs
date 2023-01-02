use bevy::prelude::*;
use std::time::Duration;

pub fn timer_tick_to_almost_finish(timer: &mut Timer) {
    let dur = timer.duration();
    timer.tick(dur - Duration::from_nanos(1));
}

pub fn timer_tick_to_finish(timer: &mut Timer) {
    let dur = timer.duration();
    timer.tick(dur + Duration::from_nanos(1));
}
