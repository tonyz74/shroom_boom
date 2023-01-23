use bevy::prelude::*;
use std::time::Duration;

#[derive(Component, Copy, Clone, Debug, Reflect)]
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


pub fn deg_to_rad(deg: f32) -> f32 {
    deg * (std::f32::consts::PI / 180.0)
}

pub fn rad_to_deg(rad: f32) -> f32 {
    rad * (180.0 / std::f32::consts::PI)
}

pub fn quat_rot2d_deg(deg: f32) -> Quat {
    quat_rot2d_rad(deg_to_rad(deg))
}

pub fn quat_rot2d_rad(rad: f32) -> Quat {
    Quat::from_rotation_z(rad)
}