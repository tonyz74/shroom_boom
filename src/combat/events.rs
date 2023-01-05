use bevy::prelude::*;

#[derive(Resource, Copy, Clone, Debug)]
pub struct HitEvent {
    pub target: Entity,
    pub damage: i32,
    pub kb: Vec2,
}