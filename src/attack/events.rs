use bevy::prelude::*;
use crate::attack::components::*;

#[derive(Resource, Copy, Clone, Debug)]
pub struct DamageInflictEvent {
    pub target: Entity,
    pub damage: Damage,
    pub kb: Knockback,
}