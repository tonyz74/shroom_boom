use bevy::prelude::*;

#[derive(Component, Resource, Copy, Clone, Debug, Reflect, FromReflect)]
pub struct CombatEvent {
    pub target: Entity,
    pub damage: i32,
    pub kb: Vec2,
}