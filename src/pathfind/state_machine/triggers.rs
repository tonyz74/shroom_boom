use bevy::prelude::*;
use seldom_state::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    enemies::Enemy,
    pathfind::{Pathfinder, walk::WalkPathfinder}
};

#[derive(Copy, Clone, Reflect, FromReflect)]
pub struct FallTrigger;

impl Trigger for FallTrigger {
    type Param<'w, 's> = Query<'w, 's,
        &'static Enemy,
        With<Pathfinder>
    >;

    fn trigger(&self, entity: Entity, enemies: &Self::Param<'_, '_>) -> bool {
        let enemy = enemies.get(entity).unwrap();
        enemy.vel.y < 0.0
    }
}


#[derive(Copy, Clone, Reflect, FromReflect)]
pub struct GroundedTrigger;

impl Trigger for GroundedTrigger {
    type Param<'w, 's> = Query<'w, 's,
        &'static KinematicCharacterControllerOutput,
        With<Pathfinder>
    >;

    fn trigger(&self, entity: Entity, outs: &Self::Param<'_, '_>) -> bool {
        if !outs.contains(entity) {
            return false;
        }

        let cc_out = outs.get(entity).unwrap();
        cc_out.grounded
    }
}


/// For MELEE / RANGED type enemies only
#[derive(Copy, Clone, Reflect, FromReflect)]
pub struct NeedsJumpTrigger;

impl Trigger for NeedsJumpTrigger {
    type Param<'w, 's> = Query<'w, 's,
        &'static WalkPathfinder,
    >;

    fn trigger(&self, entity: Entity, walks: &Self::Param<'_, '_>) -> bool {
        if !walks.contains(entity) {
            return false;
        }

        let walk = walks.get(entity).unwrap();
        walk.needs_jump
    }
}

