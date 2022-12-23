use bevy::prelude::*;
use bevy_ecs_ldtk::GridCoords;
use seldom_state::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    enemies::{Enemy, flower::FlowerEnemy},
    pathfind::{Pathfinder, PathfindingGrid}
};

#[derive(Copy, Clone, Reflect, FromReflect)]
pub struct FallTrigger;

impl Trigger for FallTrigger {
    type Param<'w, 's> = Query<'w, 's,
        &'static Enemy,
        With<FlowerEnemy>
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
        With<FlowerEnemy>
    >;

    fn trigger(&self, entity: Entity, flowers: &Self::Param<'_, '_>) -> bool {
        if !flowers.contains(entity) {
            return false;
        }

        let cc_out = flowers.get(entity).unwrap();
        cc_out.grounded
    }
}

#[derive(Copy, Clone, Reflect, FromReflect)]
pub struct NeedsJumpTrigger;

impl Trigger for NeedsJumpTrigger {
    type Param<'w, 's> = Query<'w, 's,
        &'static CrawlPathfinder,
        With<FlowerEnemy>
    >;

    fn trigger(&self, entity: Entity, enemies: &Self::Param<'_, '_>) -> bool {
        let crawl = enemies.get(entity).unwrap();

        crawl.needs_jump
    }
}

#[derive(Copy, Clone, Component, Reflect)]
pub struct Jump;

#[derive(Copy, Clone, Component, Reflect)]
pub struct Run;

#[derive(Copy, Clone, Component, Reflect)]
pub struct Fall;

pub fn snake_enemy_state_machine() -> StateMachine {
    StateMachine::new(Run)
        .trans::<Run>(NotTrigger(GroundedTrigger), Fall)
        .trans::<Fall>(GroundedTrigger, Run)
        .trans::<Run>(NeedsJumpTrigger, Jump)
        .trans::<Jump>(FallTrigger, Fall)
}
use crate::enemies::flower;
use crate::pathfind::crawl::CrawlPathfinder;