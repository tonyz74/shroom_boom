use bevy::prelude::*;

use bevy_rapier2d::{na, rapier as rp};

pub const PLAYER_COLLIDER_CAPSULE: rp::geometry::Capsule = rp::geometry::Capsule {
    segment: rp::geometry::Segment {
        a: na::OPoint::<f32, na::Const<2>>::new(8.0, 0.0),
        b: na::OPoint::<f32, na::Const<2>>::new(-8.0, 0.0),
    },
    radius: 22.0
};

pub const PLAYER_SIZE_PX: Vec2 = Vec2::new(128.0, 128.0);

pub const PLAYER_DASH_LENGTH: f32 = 0.13;
pub const PLAYER_SHOOT_EXPIRATION_TIME: f32 = 0.7;

pub const PLAYER_RUN_SPEED: f32 = 5.0;
pub const PLAYER_JUMP_SPEED: f32 = 14.2;
pub const PLAYER_FALL_GRAVITY: f32 = -40.0;
pub const PLAYER_TERMINAL_VELOCITY: f32 = -20.0;
pub const PLAYER_COYOTE_TIME: f32 = 0.16;
pub const PLAYER_JUMP_BUFFER_TIME: f32 = 0.08;



// STATS

pub const HEALTH_LEVELS: [i32; 6] = [
    100,
    115,
    130,
    155,
    180,
    200
];

pub const AMMO_LEVELS: [i32; 6] = [
    50,
    60,
    70,
    80,
    90,
    100
];

pub const DASH_LEVELS: [(f32, f32, i32); 6] = [
    (1.0, 20.0, 1),
    (1.0, 22.0, 2),
    (0.9, 25.0, 3),
    (0.7, 26.0, 3),
    (0.5, 30.0, 4),
    (0.3, 30.0, 4)
];

pub const SLASH_LEVELS: [(f32, i32); 6] = [
    (0.5, 1),
    (0.5, 2),
    (0.4, 3),
    (0.4, 4),
    (0.3, 5),
    (0.2, 5),
];

pub const SHOOT_LEVELS: [(f32, f32, i32); 6] = [
    (0.8, 10.0, 2),
    (0.7, 11.0, 3),
    (0.6, 12.0, 4),
    (0.5, 13.0, 5),
    (0.4, 14.0, 6),
    (0.3, 16.0, 7),
];