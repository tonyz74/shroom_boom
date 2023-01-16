use std::ops::Range;
use bevy::prelude::*;

pub const EXPLOSION_DIAMETER: f32 = 64.0;
pub const EXPLOSION_RADIUS: f32 = EXPLOSION_DIAMETER / 2.0;
pub const EXPLOSION_DURATION: f32 = 0.4;
pub const EXPLOSION_EFFECTIVE_DURATION: f32 = 0.15;

pub const SPORE_CLOUD_SIZE: Vec2 = Vec2::splat(32.0);
pub const SPORE_CLOUD_DURATION: f32 = 8.0;
pub const SPORE_CLOUD_DAMAGE_RATE: f32 = 0.8;
pub const SPORE_CLOUD_PARTICLE_SPAWN_RATE: f32 = 0.6;
pub const SPORE_CLOUD_COLOR: Color = Color::rgba(0.0, 0.2, 1.0, 0.4);
pub const SPORE_CLOUD_SPORE_ROTATION_RANGE: Range<f32> = -30.0..30.0;
pub const SPORE_CLOUD_SPORE_SCALE_RANGE: Range<f32> = -0.5..0.0;
