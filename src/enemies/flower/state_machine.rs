use bevy::prelude::*;
use seldom_state::prelude::*;
use bevy_rapier2d::prelude::*;

use super::SnakeEnemy;

#[derive(Copy, Clone, Reflect, FromReflect)]
pub struct FallTrigger;

impl Trigger for FallTrigger {
    type Param<'w, 's> = Query<'w, 's,
        &'static KinematicCharacterControllerOutput,
        With<SnakeEnemy>
    >;

    fn trigger(&self, entity: Entity, snakes: &Self::Param<'_, '_>) -> bool {
        if let Ok(cc_out) = snakes.get(entity) {
            !cc_out.grounded
        } else {
            false
        }
    }
}

#[derive(Copy, Clone, Reflect, FromReflect)]
pub struct GroundedTrigger;

impl Trigger for GroundedTrigger {
    type Param<'w, 's> = Query<'w, 's,
        &'static KinematicCharacterControllerOutput,
        With<SnakeEnemy>
    >;

    fn trigger(&self, entity: Entity, snakes: &Self::Param<'_, '_>) -> bool {
        let cc_out = snakes.get(entity).unwrap();
        cc_out.grounded
    }
}

#[derive(Copy, Clone, Component, Reflect)]
pub struct Idle;

#[derive(Copy, Clone, Component, Reflect)]
pub struct Fall;

pub fn snake_enemy_state_machine() -> StateMachine {
    StateMachine::new(Idle)
        .trans::<Idle>(FallTrigger, Fall)
        .trans::<Fall>(GroundedTrigger, Idle)
}