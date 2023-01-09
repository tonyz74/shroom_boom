use bevy::prelude::*;
use bevy::math::Vec3Swizzles;
use bevy_rapier2d::prelude::*;
use crate::combat::CombatLayerMask;
use crate::player::Player;
use crate::util::Facing;

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

fn attack_direction_between(self_pos: Vec2, target_pos: Vec2) -> AttackDirection {
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

pub fn get_closest_target(
    self_ent: Entity,
    self_combat_layer: CombatLayerMask,
    span: f32,
    transforms: &Query<&GlobalTransform>,
    combat_layers: &Query<&CombatLayerMask>,
    rapier: &RapierContext,
) -> Option<(Vec2, AttackDirection)> {

    let self_pos = transforms.get(self_ent).unwrap().translation().xy();
    let mut closest_target: Option<Vec2> = None;

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

            if let Some(best) = closest_target {
                if best.distance(self_pos) > target_pos.distance(self_pos) {
                    closest_target = Some(target_pos);
                }
            } else {
                closest_target = Some(target_pos);
            }

            true
        }
    );

    if let Some(target) = closest_target {
        Some((target, attack_direction_between(self_pos, target)))
    } else {
        None
    }
}

pub fn change_facing_for_direction(player: &mut Player, dir: AttackDirection) {
    match dir {
        AttackDirection::Left | AttackDirection::DownLeft | AttackDirection::UpLeft => {
            player.facing = Facing::Left
        },
        AttackDirection::Right | AttackDirection::DownRight | AttackDirection::UpRight => {
            player.facing = Facing::Right
        },
        _ => {}
    }
}

pub fn direction_for_facing(facing: Facing) -> AttackDirection {
    match facing {
        Facing::Left => AttackDirection::Left,
        Facing::Right => AttackDirection::Right
    }
}

pub fn direction_to_vec(dir: AttackDirection) -> Vec2 {
    match dir {
        AttackDirection::Left => Vec2::new(-1.0, 0.0),
        AttackDirection::Right => Vec2::new(1.0, 0.0),
        _ => Vec2::ZERO
    }
}