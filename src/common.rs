use bevy::prelude::*;

#[derive(SystemLabel)]
pub enum UpdateStage {
    Physics,
    GameLogic
}

pub const PHYSICS_STEPS_PER_SEC: f64 = 60.0;
pub const PHYSICS_STEP_DELTA: f32 = 1.0 / 60.0;