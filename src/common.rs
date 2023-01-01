use bevy::prelude::*;

#[derive(SystemLabel)]
pub enum UpdateStage {
    Physics,
    GameLogic
}

pub const PHYSICS_STEPS_PER_SEC: f64 = 60.0;
pub const PHYSICS_STEP_DELTA: f32 = 1.0 / 60.0;

#[derive(Default, Clone, Component, Deref, DerefMut, Debug)]
pub struct AnimTimer(Timer);

impl AnimTimer {
    pub fn from_seconds(s: f32) -> Self {
        AnimTimer(Timer::from_seconds(s, TimerMode::Repeating))
    }
}

#[derive(Debug, Default, Clone)]
pub struct Anim {
    pub tex: Handle<TextureAtlas>,
    pub speed: f32
}

impl Anim {
    pub fn new(handle: Handle<TextureAtlas>, speed: f32) -> Self {
        Anim {
            tex: handle,
            speed
        }
    }
}