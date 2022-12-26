use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::attack::{AttackStrength, CombatLayerMask, HitEvent};
use crate::common::AnimTimer;

#[derive(Component, Default)]
pub struct ProjectileAttack {
    pub vel: Vec2
}

#[derive(Bundle, Default)]
pub struct ProjectileAttackBundle {
    pub anim_timer: AnimTimer,
    pub sprite_sheet: SpriteSheetBundle,
    pub collider: Collider,
    pub sensor: Sensor,
    pub attack: ProjectileAttack,
    pub strength: AttackStrength,
    pub combat_layer: CombatLayerMask,
    pub controller: KinematicCharacterController
}

impl ProjectileAttackBundle {
    pub fn from_size(size: Vec2) -> Self {
        Self {
            collider: Collider::cuboid(size.x / 2., size.y / 2.),
            sensor: Sensor,
            controller: KinematicCharacterController {
                offset: CharacterLength::Relative(0.02),
                filter_flags: QueryFilterFlags::EXCLUDE_SENSORS | QueryFilterFlags::EXCLUDE_FIXED,
                ..default()
            },
            ..default()
        }
    }
}

pub fn move_projectile_attacks(
    mut proj: Query<(&ProjectileAttack, &mut KinematicCharacterController)>
) {
    for (proj, mut cc) in proj.iter_mut() {
        cc.translation = Some(proj.vel);
    }
}

pub fn remove_projectiles_on_impact(
    mut commands: Commands,
    projectiles: Query<(
        Entity,
        &GlobalTransform,
        &Collider,
        &CombatLayerMask,
    ), With<ProjectileAttack>>,
    rapier: Res<RapierContext>,

    combat_layers: Query<&CombatLayerMask>,
    mut hit_events: EventWriter<HitEvent>
) {
    for (entity, transform, collider, proj_combat_layer) in projectiles.iter() {
        let proj_pos = transform.translation();
        // See if it has any intersections with walls
        let mut should_despawn = false;

        rapier.intersections_with_shape(
            Vect::new(proj_pos.x, proj_pos.y),
            Rot::default(),
            collider,
            QueryFilter {
                flags: QueryFilterFlags::ONLY_FIXED
                    | QueryFilterFlags::EXCLUDE_SENSORS,
                ..default()
            },
            |_| {
                should_despawn = true;
                true
            }
        );

        rapier.intersections_with_shape(
            Vect::new(proj_pos.x, proj_pos.y),
            Rot::default(),
            collider,
            QueryFilter {
                flags: QueryFilterFlags::ONLY_KINEMATIC,
                ..default()
            },
            |ent| {
                if let Ok(layer) = combat_layers.get(ent) {
                    if !layer.is_ally_with(*proj_combat_layer) {
                        hit_events.send(HitEvent {
                            target: ent,
                            damage: 2,
                            kb: Vec2::new(4.0, 2.0)
                        });
                        should_despawn = true;
                    }
                }

                true
            }
        );

        if should_despawn {
            commands.entity(entity).despawn();
        }
   }
}
