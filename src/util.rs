use bevy::prelude::*;
use std::time::Duration;

#[derive(Copy, Clone, Debug, Reflect)]
pub enum Facing {
    Left,
    Right
}

impl Default for Facing {
    fn default() -> Self {
        Facing::Right
    }
}

pub fn timer_tick_to_almost_finish(timer: &mut Timer) {
    let dur = timer.duration();
    timer.tick(dur - Duration::from_nanos(1));
}

pub fn timer_tick_to_finish(timer: &mut Timer) {
    let dur = timer.duration();
    timer.tick(dur + Duration::from_nanos(1));
}
