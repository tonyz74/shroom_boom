use bevy::prelude::*;
use bevy::math::Vec3Swizzles;
use bevy_rapier2d::prelude::*;
use crate::combat::{CombatLayerMask, ProjectileAttack};
use crate::level::consts::SOLIDS_INTERACTION_GROUP;

use crate::util::{Facing, FacingX};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Component)]
pub struct Untargetable;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AttackDirection {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight
}

impl Default for AttackDirection {
    fn default() -> Self {
        Self::Up
    }
}

pub fn attack_direction_between(self_pos: Vec2, target_pos: Vec2) -> AttackDirection {
    let dir = (target_pos - self_pos).normalize_or_zero();
    if dir == Vec2::ZERO {
        return AttackDirection::Up;
    }

    // Angles for each direction vector in degrees
    let angles = &[
        (Vec2::new(1.0, 0.0).normalize(), 0.0, AttackDirection::Right),
        (Vec2::new(1.0, 1.0).normalize(), 45.0, AttackDirection::UpRight),
        (Vec2::new(0.0, 1.0).normalize(), 90.0, AttackDirection::Up),
        (Vec2::new(-1.0, 1.0).normalize(), 135.0, AttackDirection::UpLeft),
        (Vec2::new(-1.0, 0.0).normalize(), 180.0, AttackDirection::Left),
        (Vec2::new(-1.0, -1.0).normalize(), 225.0, AttackDirection::DownLeft),
        (Vec2::new(0.0, -1.0).normalize(), 270.0, AttackDirection::Down),
        (Vec2::new(1.0, -1.0).normalize(), 315.0, AttackDirection::DownRight),
        (Vec2::new(1.0, 0.0).normalize(), 360.0, AttackDirection::Right),
    ];

    let pos_x = angles[0].0;
    let mut angle = pos_x.dot(dir).acos() * (180.0 / std::f32::consts::PI);

    if self_pos.y > target_pos.y {
        angle = 360.0 - angle;
    }

    let best = {
        let mut best_diff = f32::MAX;
        let mut best_pick = AttackDirection::Up;

        for (_, dir_angle, dir) in angles {
            if (dir_angle - angle).abs() < best_diff {
                best_diff = (dir_angle - angle).abs();
                best_pick = *dir;
            }
        }

        best_pick
    };

    best
}


fn has_clear_line_of_sight(_commands: &mut Commands, start: Vec2, end: Vec2, rapier: &RapierContext) -> bool {
    let hit = rapier.cast_ray(
        start,
        (end - start).normalize(),
        end.distance(start).abs(),
        true,
        QueryFilter {
            flags: QueryFilterFlags::ONLY_FIXED | QueryFilterFlags::EXCLUDE_SENSORS,
            groups: Some(SOLIDS_INTERACTION_GROUP),
            ..default()
        }
    );

    // println!("hit: {:?}", hit);
    //
    // if let Some((e, t)) = hit {
    //     commands.entity(e).despawn_recursive();
    // }

    hit.is_none()
}


pub fn get_closest_target(
    commands: &mut Commands,
    self_ent: Entity,
    self_combat_layer: CombatLayerMask,
    span: f32,
    ignore_projectiles: bool,
    transforms: &Query<&GlobalTransform>,
    combat_layers: &Query<&CombatLayerMask>,
    disabled: &Query<&Untargetable>,
    projectiles: &Query<&ProjectileAttack>,
    prefer_los: bool,
    rapier: &RapierContext,
) -> Option<(Vec2, AttackDirection)> {

    let self_pos = transforms.get(self_ent).unwrap().translation().xy();
    let mut closest_target: Option<Vec2> = None;
    let mut closest_los_target: Option<Vec2> = None;

    rapier.intersections_with_shape(
        self_pos,
        Rot::default(),
        &Collider::cuboid(span, span),
        QueryFilter {
            predicate: Some(&|x: Entity| {
                if let Ok(x) = combat_layers.get(x) {
                    return !x.is_ally_with(self_combat_layer);
                }

                false
            }),
            exclude_rigid_body: Some(self_ent),
            flags: QueryFilterFlags::ONLY_KINEMATIC,
            ..default()
        },
        |colliding_entity| {
            let target_pos = transforms.get(colliding_entity).unwrap().translation().xy();

            if disabled.contains(colliding_entity) ||
                (ignore_projectiles && projectiles.contains(colliding_entity)) {
                return true;
            }

            if let Some(best) = closest_target {
                if best.distance(self_pos) > target_pos.distance(self_pos) {
                    closest_target = Some(target_pos);
                }
            } else {
                closest_target = Some(target_pos);
            }

            if prefer_los {
                println!("yess");
                if let Some(best) = closest_los_target {
                    if best.distance(self_pos) > target_pos.distance(self_pos) {
                        println!("candidate: {:?}", target_pos);
                        if has_clear_line_of_sight(commands, self_pos, target_pos, &rapier) {
                            closest_los_target = Some(target_pos);
                        }
                    }
                } else {
                    if has_clear_line_of_sight(commands, self_pos, target_pos, &rapier) {
                        closest_los_target = Some(target_pos);
                    }
                }
            }

            true
        }
    );

    let rv = if let Some(target) = closest_target {
        Some((target, attack_direction_between(self_pos, target)))
    } else {
        None
    };

    println!("{:?} {:?}", closest_target, closest_los_target);

    if prefer_los {
        match closest_los_target {
            Some(target) => Some((target, attack_direction_between(self_pos, target))),
            None => rv
        }
    } else {
        rv
    }
}

pub fn change_facing_for_direction(facing: &mut Facing, dir: AttackDirection) {
    match dir {
        AttackDirection::Left | AttackDirection::DownLeft | AttackDirection::UpLeft => {
            facing.x = FacingX::Left
        },
        AttackDirection::Right | AttackDirection::DownRight | AttackDirection::UpRight => {
            facing.x = FacingX::Right
        },
        _ => {}
    }
}

pub fn direction_for_facing(facing: Facing) -> AttackDirection {
    match facing.x {
        FacingX::Left => AttackDirection::Left,
        FacingX::Right => AttackDirection::Right
    }
}

pub fn direction_to_vec(dir: AttackDirection) -> Vec2 {
    match dir {
        AttackDirection::Left => Vec2::new(-1.0, 0.0),
        AttackDirection::UpLeft => Vec2::new(-1.0, 0.0),
        AttackDirection::DownLeft => Vec2::new(-1.0, 0.0),

        AttackDirection::Right => Vec2::new(1.0, 0.0),
        AttackDirection::UpRight => Vec2::new(1.0, 0.0),
        AttackDirection::DownRight => Vec2::new(1.0, 0.0),

        _ => Vec2::ZERO
    }
}