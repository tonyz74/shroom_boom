use bevy::prelude::*;
use seldom_state::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    enemies::Enemy,
    pathfind::{Pathfinder, walk::WalkPathfinder}
};
use crate::combat::HurtAbility;
use crate::pathfind::RangedPathfinder;

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

#[derive(Copy, Clone, Reflect, FromReflect)]
pub struct StopHurtTrigger;

impl Trigger for StopHurtTrigger {
    type Param<'w, 's> = Query<'w, 's, (&'static WalkPathfinder, &'static HurtAbility)>;

    fn trigger(&self, entity: Entity, walk: &Self::Param<'_, '_>) -> bool {
        if !walk.contains(entity) {
            return false;
        }

        let (walk, hurt) = walk.get(entity).unwrap();
        walk.grounded && hurt.can_stop_hurting()
    }
}

#[derive(Copy, Clone, Reflect, FromReflect)]
pub struct ShootTrigger;

impl Trigger for ShootTrigger {
    type Param<'w, 's> = Query<'w, 's, &'static RangedPathfinder>;

    fn trigger(
        &self,
        entity: Entity,
        pathfinders: &Self::Param<'_, '_>
    ) -> bool {
        if !pathfinders.contains(entity) {
            return false;
        }

        let ranged = pathfinders.get(entity).unwrap();

        ranged.shoot_target.is_some() && ranged.shoot_cooldown.finished()
    }
}


#[derive(Copy, Clone, Reflect, FromReflect)]
pub struct HitWallTrigger;

impl Trigger for HitWallTrigger {
    type Param<'w, 's> = (
        Query<'w, 's, &'static Enemy>,
        Query<'w, 's, &'static KinematicCharacterControllerOutput>
    );

    fn trigger(
        &self,
        entity: Entity,
        (enemies, cc_outs): &Self::Param<'_, '_>
    ) -> bool {
        if !enemies.contains(entity) {
            return false;
        }

        let enemy = enemies.get(entity).unwrap();

        if cc_outs.contains(entity) {
            let out = cc_outs.get(entity).unwrap();
            if out.desired_translation != Vec2::ZERO && out.effective_translation == Vec2::ZERO {
                return true;
            }
        }

        let ok = enemy.vel.length() <= 1.0;
        ok
    }
}


#[derive(Copy, Clone, Reflect, FromReflect)]
pub struct HitHeadTrigger;

impl Trigger for HitHeadTrigger {
    type Param<'w, 's> = (
        Query<'w, 's, (&'static Enemy, &'static GlobalTransform, &'static Pathfinder)>,
        Res<'w, RapierContext>
    );

    fn trigger(&self, entity: Entity, (q, ctx): &Self::Param<'_, '_>) -> bool {
        if !q.contains(entity) {
            return false;
        }

        let (enemy, tf, path) = q.get(entity).unwrap();
        let pos = Vec2::new(tf.translation().x, tf.translation().y);

        let span = path.bb.half_extents.x;

        let origins = [
            Vect::new(pos.x - span, pos.y),
            Vect::new(pos.x + 0.00, pos.y),
            Vect::new(pos.x + span, pos.y),
        ];

        for origin in origins {
            let rc = ctx.cast_ray(
                origin,
                Vect::new(0.0, 1.0).normalize(),
                path.bb.half_extents.y + enemy.vel.y + 1.0,
                true,
                QueryFilter {
                    flags: QueryFilterFlags::ONLY_FIXED | QueryFilterFlags::EXCLUDE_SENSORS,
                    ..default()
                }
            );

            if rc.is_some() {
                return true;
            }
        }

        false
    }
}