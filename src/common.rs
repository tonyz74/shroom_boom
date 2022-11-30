use bevy::prelude::*;

#[derive(SystemLabel)]
pub enum UpdateStage {
    Physics,
    GameLogic
}

pub const PHYSICS_STEPS_PER_SEC: f64 = 60.0;

#[derive(Component, Deref, DerefMut)]
pub struct AnimTimer(Timer);

impl AnimTimer {
    pub fn from_seconds(s: f32) -> Self {
        AnimTimer(Timer::from_seconds(s, TimerMode::Repeating))
    }
}
