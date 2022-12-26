use bevy::prelude::*;

use bevy_rapier2d::{na, rapier as rp};

pub const PLAYER_COLLIDER_CAPSULE: rp::geometry::Capsule = rp::geometry::Capsule {
    segment: rp::geometry::Segment {
        a: na::OPoint::<f32, na::Const<2>>::new(0.0, 4.0),
        b: na::OPoint::<f32, na::Const<2>>::new(0.0, -4.0),
    },
    radius: 20.0
};

pub const PLAYER_SIZE_PX: Vec2 = Vec2::new(64.0, 64.0);

pub const PLAYER_ATTACK_COOLDOWN: f32 = 0.2;

pub const PLAYER_DASH_LENGTH: f32 = 0.1;
pub const PLAYER_DASH_COOLDOWN: f32 = 0.3;
pub const PLAYER_DASH_SPEED: f32 = 30.0;

pub const PLAYER_RUN_SPEED: f32 = 5.0;

pub const PLAYER_JUMP_SPEED: f32 = 14.0;

pub const PLAYER_FALL_GRAVITY: f32 = -40.0;

pub const PLAYER_TERMINAL_VELOCITY: f32 = -20.0;

pub const PLAYER_COYOTE_TIME: f32 = 0.15;
pub const PLAYER_JUMP_BUFFER_TIME: f32 = 0.1;
