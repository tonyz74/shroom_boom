use bevy::prelude::*;
use bevy_debug_text_overlay::screen_print;
use bevy_rapier2d::prelude::*;
use seldom_state::prelude::Done;

use crate::combat::{CombatLayerMask, ProjectileAttackBundle};
use crate::state::GameState;
use crate::pathfind::{Pathfinder, WalkPathfinder, walk_pathfinder_jump_if_needed, Patrol, walk_pathfinder_get_suitable_target};
use crate::enemies::Enemy;
use crate::entity_states::*;
use crate::level::door::DoorTile;
use crate::level::one_way::OneWayTile;
use crate::level::solid::SolidTile;
use crate::util::{Facing, FacingX, quat_rot2d_rad};

#[derive(Component, Clone)]
pub struct RangedPathfinder {
    pub shoot_pause: Timer,
    pub shoot_startup: Timer,
    pub shoot_cooldown: Timer,

    pub shoot_target: Option<Vec2>,
    pub shoot_offset: Vec2,

    /// Maximum angle accepted (relative to the the UP vector) for a shot, in radians.
    pub max_shoot_angle: f32,
    pub max_shoot_distance: f32,
    pub projectile: ProjectileAttackBundle,
    pub extra_spawn: fn(&mut Commands, Entity)
}

impl Default for RangedPathfinder {
    fn default() -> Self {
        Self {
            shoot_pause: Default::default(),
            shoot_startup: Default::default(),
            shoot_cooldown: Default::default(),
            shoot_offset: Vec2::ZERO,
            shoot_target: None,
            max_shoot_angle: 360.0,
            max_shoot_distance: 256.0,
            projectile: Default::default(),
            extra_spawn: |_, _| {}
        }
    }
}

pub fn register_ranged_pathfinders(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(ranged_pathfinder_move)
            .with_system(ranged_pathfinder_tick_shoot_cooldown)
            .with_system(ranged_pathfinder_add_shoot)
            .with_system(ranged_pathfinder_shoot)
    );
}

pub fn has_clear_line_of_sight(
    commands: &mut Commands,
    start: Vec2,
    end: Vec2,
    shape: &Collider,
    one_ways: &Query<Entity, With<OneWayTile>>,
    rapier: &Res<RapierContext>,
) -> bool {
    let distance = start.distance(end);

    let out = rapier.cast_shape(
        start,
        Rot::default(),
        (end - start).normalize(),
        shape,
        distance,
        QueryFilter {
            flags: QueryFilterFlags::ONLY_FIXED | QueryFilterFlags::EXCLUDE_SENSORS,
            ..default()
        }
    );

    if let Some(o) = out {
        if one_ways.contains(o.0) {
            return true;
        }
    }

    out.is_none()
}

pub fn is_valid_shot(
    commands: &mut Commands,
    start: Vec2,
    end: Vec2,
    one_ways: &Query<Entity, With<OneWayTile>>,
    ranged: &RangedPathfinder,
    rapier: &Res<RapierContext>
) -> bool {
    let diff = start - end;
    let proj_collider = &ranged.projectile.collider;

    let shoot_angle = (diff.y / diff.length()).asin().abs();

    let ok = start.distance(end).abs() <= ranged.max_shoot_distance
        && has_clear_line_of_sight(commands, start, end, proj_collider, one_ways, rapier)
        && shoot_angle <= ranged.max_shoot_angle;

    ok
}

pub fn ranged_pathfinder_move(
    mut commands: Commands,
    transforms: Query<&GlobalTransform>,
    combat_layers: Query<&CombatLayerMask>,
    mut ranged_pathfinders: Query<&mut RangedPathfinder>,
    jumping: Query<&Jump>,
    mut pathfinders: Query<(
        Entity,
        &Collider,
        &mut Enemy,
        &mut Pathfinder,
        &mut WalkPathfinder,
        &mut Facing,
        &mut Patrol
    ), (Without<Hurt>, Without<Shoot>, Without<Die>, With<RangedPathfinder>)>,
    one_ways: Query<Entity, With<OneWayTile>>,
    rapier: Res<RapierContext>
) {
    let _ = rapier;

    for (ent, collider, mut enemy, mut pathfinder, mut walk, mut facing, mut patrol) in pathfinders.iter_mut() {
        if !pathfinder.active {
            continue;
        }

        patrol.can_patrol = true;
        walk.needs_jump = false;

        let transform = transforms.get(ent).unwrap();

        let mut ranged = ranged_pathfinders.get_mut(ent).unwrap();
        ranged.shoot_target = None;

        let self_pos = Vec2::new(
            transform.translation().x,
            transform.translation().y
        );

        // If there is a player within the enemy's region
        if let Some(mut target) = pathfinder.target {
            // target = walk_pathfinder_get_suitable_target(self_pos, target, &pathfinder);

            let is_valid_shot = is_valid_shot(&mut commands, self_pos, target, &one_ways, &ranged, &rapier);

            if (target.x - self_pos.x).abs() <= 2.0 {
                if patrol.lost_target {
                    pathfinder.target = None;
                }

                if is_valid_shot {
                    ranged.shoot_target = Some(target);
                }

                enemy.vel.x = 0.0;
                continue;
            }

            if patrol.lost_target || !is_valid_shot || jumping.contains(ent) {
                let diff = target - self_pos;
                let dir = Vec2::new(diff.x, 0.0).normalize();

                enemy.vel.x = dir.x * pathfinder.speed;

                if dir.x < 0.0 {
                    facing.x = FacingX::Left;
                } else if dir.x > 0.0 {
                    facing.x = FacingX::Right;
                }

                walk_pathfinder_jump_if_needed(
                    Vec2::new(self_pos.x, self_pos.y),
                    dir.into(),
                    collider,
                    &mut enemy,
                    &pathfinder,
                    &mut walk,
                    &rapier
                );
            } else {
                ranged.shoot_target = Some(target);
            }

        } else {
            let new_region = pathfinder.region.expanded_by(ranged.max_shoot_distance);
            let half_extents = new_region.extents() / 2.0;

            let mut opposition_pos = None;
            let self_combat_layer = combat_layers.get(ent).unwrap();

            rapier.intersections_with_shape(
                self_pos,
                Rot::default(),
                &Collider::cuboid(half_extents.x, half_extents.y),
                QueryFilter {
                    flags: QueryFilterFlags::ONLY_KINEMATIC,
                    ..default()
                },
                |hit| {
                    if let Ok(combat_layer) = combat_layers.get(hit) {
                        if !combat_layer.is_ally_with(*self_combat_layer) {
                            opposition_pos = Some(transforms.get(hit).unwrap().translation());
                        }
                    }

                    true
                }
            );

            if let Some(opposition_pos) = opposition_pos {
                let opposition_pos = Vec2::new(
                    opposition_pos.x,
                    opposition_pos.y
                );

                if is_valid_shot(&mut commands, self_pos, opposition_pos, &one_ways, &ranged, &rapier) {
                    ranged.shoot_target = Some(opposition_pos);
                    patrol.can_patrol = false;
                }
            }
        }

    }
}

pub fn ranged_pathfinder_tick_shoot_cooldown(
    time: Res<Time>,
    mut q: Query<&mut RangedPathfinder>
) {
    for mut ranged in q.iter_mut() {
        ranged.shoot_cooldown.tick(time.delta());
    }
}

pub fn ranged_pathfinder_add_shoot(
    mut q: Query<(&GlobalTransform, &mut Enemy, &mut Facing, &mut RangedPathfinder), Added<Shoot>>
) {
    for (tf, mut enemy, mut facing, mut ranged) in q.iter_mut() {
        let pos = tf.translation();

        enemy.vel.x = 0.0;
        ranged.shoot_startup.reset();

        if let Some(target) = ranged.shoot_target {
            let dx = Vec2::new(target.x - pos.x, 0.0).normalize_or_zero().x;

            if dx < 0.0 {
                facing.x = FacingX::Left;
            } else if dx > 0.0 {
                facing.x = FacingX::Right;
            }
        }
    }
}

pub fn ranged_pathfinder_shoot(
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(
        Entity,
        &GlobalTransform,
        &mut Enemy,
        &mut RangedPathfinder
    ), With<Shoot>>
) {
    for (entity, tf, mut enemy, mut ranged) in q.iter_mut() {
        ranged.shoot_startup.tick(time.delta());

        if ranged.shoot_startup.just_finished() {

            ranged.shoot_pause.reset();
            ranged.shoot_cooldown.reset();

            let pos = Vec2::new(
                tf.translation().x,
                tf.translation().y
            );

            enemy.vel.x = 0.0;

            if let Some(target) = ranged.shoot_target {
                let mut proj = ranged.projectile.clone();
                let adjusted_pos = pos + ranged.shoot_offset;

                let vel: Vec2 = (target - adjusted_pos).normalize() * proj.attack.speed;
                proj.attack.vel = vel;
                proj.sprite_sheet.transform.translation = adjusted_pos.extend(5.0);
                proj.sprite_sheet.transform.rotation = quat_rot2d_rad(-vel.angle_between(Vec2::X));

                let eid = commands.spawn(proj).id();
                (ranged.extra_spawn)(&mut commands, eid);
            }

        } else if ranged.shoot_startup.finished() {
            ranged.shoot_pause.tick(time.delta());

            if ranged.shoot_pause.just_finished() {
                commands.entity(entity).insert(Done::Success);
            }
        }

    }
}