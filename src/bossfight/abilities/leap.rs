use bevy::prelude::*;
use seldom_state::prelude::*;
use bevy::math::Vec3Swizzles;
use bevy::time::FixedTimestep;
use bevy_rapier2d::control::KinematicCharacterController;
use bevy_rapier2d::prelude::QueryFilterFlags;

use crate::bossfight::{Boss, BossConfig};
use crate::bossfight::enraged::EnragedAttackMove;
use crate::bossfight::state_machine::{AbilityStartup, Leap};
use crate::combat::Immunity;
use crate::common::{PHYSICS_STEP_DELTA, PHYSICS_STEPS_PER_SEC};
use crate::enemies::Enemy;
use crate::state::GameState;
use crate::util::Facing;

#[derive(Component, Debug, Clone)]
pub struct LeapAbility {
    pub rotate_lag: Timer
}

impl Default for LeapAbility {
    fn default() -> Self {
        Self {
            rotate_lag: Timer::from_seconds(0.1, TimerMode::Once)
        }
    }
}

pub fn register_leap_ability(app: &mut App) {
    app
        .add_system_set(
            SystemSet::on_update(GameState::Gameplay)
                .with_system(start_leaping)
                .with_system(leap_update)
        )
        .add_system_set(
            SystemSet::on_update(GameState::Gameplay)
                .with_run_criteria(FixedTimestep::steps_per_second(PHYSICS_STEPS_PER_SEC))
                .with_system(leap_rotate)
        );
}

fn start_leaping(
    mut q: Query<(
        &mut Immunity,
        &mut LeapAbility,
        &mut KinematicCharacterController,
        &mut Boss,
    ), Added<AbilityStartup>>
) {
    if q.is_empty() {
        return;
    }

    let (mut immunity, mut leap, mut cc, mut boss) = q.single_mut();
    if boss.current_move() != EnragedAttackMove::Leap {
        return;
    }

    cc.filter_flags = QueryFilterFlags::EXCLUDE_SENSORS | QueryFilterFlags::EXCLUDE_FIXED;
    immunity.is_immune = true;
    boss.facing = Facing::Left;
    leap.rotate_lag.reset();
}

fn leap_update(
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(
        Entity,
        &GlobalTransform,
        &mut Transform,
        &mut Enemy,
        &mut Immunity,
        &mut KinematicCharacterController,
        &mut LeapAbility,
        &BossConfig
    ), With<Leap>>
) {
   if q.is_empty() {
       return;
   }

    let (entity, tf, mut mov, mut enemy, mut immunity, mut cc, mut leap, cfg) = q.single_mut();

    leap.rotate_lag.tick(time.delta());
    let pos = tf.translation().xy();

    if pos.distance(cfg.hover_base).abs() <= 16.0 {
        commands.entity(entity).insert(Done::Success);

        immunity.is_immune = false;
        enemy.vel = Vec2::ZERO;
        mov.translation = cfg.hover_base.extend(mov.translation.z);
        mov.rotation = Quat::from_rotation_z(0.0);

        cc.filter_flags = QueryFilterFlags::EXCLUDE_SENSORS;

        return;
    }

    enemy.vel = (cfg.hover_base - pos).normalize() * 24.0;
}

fn leap_rotate(
    mut q: Query<(
        &mut Transform,
        &LeapAbility
    ), With<Leap>>
) {
    if q.is_empty() {
        return;
    }

    let (mut transform, leap) = q.single_mut();
    let (_, _, rot) = transform.rotation.to_euler(EulerRot::XYZ);

    if (rot * 57.29577).abs() <= 1.0 {
        transform.rotation = Quat::from_rotation_z(0.0);
    } else if leap.rotate_lag.finished() {
        transform.rotate_z((3.14 / 180.0) * (-360.0 * PHYSICS_STEP_DELTA));
    }
}