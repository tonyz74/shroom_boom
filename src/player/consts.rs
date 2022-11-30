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

pub const PLAYER_ANIM_SPEED: f32 = 0.2;
