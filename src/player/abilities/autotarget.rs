use bevy::prelude::*;
use bevy::math::Vec3Swizzles;
use bevy_rapier2d::prelude::*;
use crate::combat::CombatLayerMask;

#[derive(Copy, Clone, Debug)]
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

    AttackDirection::Down
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