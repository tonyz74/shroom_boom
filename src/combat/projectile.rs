use std::collections::HashSet;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::combat::{AttackStrength, CombatLayerMask, HitEvent, Immunity};
use crate::common::AnimTimer;

#[derive(Copy, Clone, Debug, Component, Resource)]
pub enum ProjectileCollisionType {
    SolidTile,
    Entity
}

#[derive(Resource, Clone, Copy, Debug)]
pub struct ProjectileCollisionEvent {
    pub proj: Entity,
    pub collision: Entity,
    pub collision_type: ProjectileCollisionType
}

#[derive(Component, Default, Clone, Debug)]
pub struct ProjectileAttack {
    pub vel: Vec2,
    pub speed: f32
}

#[derive(Bundle, Default, Clone)]
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

pub fn projectile_hit_targets(
    immunities: Query<&Immunity>,
    transforms: Query<&GlobalTransform>,
    projectiles: Query<(
        Entity,
        &Collider,
        &CombatLayerMask,
        &AttackStrength,
        &ProjectileAttack
    )>,
    rapier: Res<RapierContext>,

    combat_layers: Query<&CombatLayerMask>,
    mut hit_events: EventWriter<HitEvent>,
    mut proj_collision_events: EventWriter<ProjectileCollisionEvent>
) {
    for (entity, collider, proj_combat_layer, strength, proj) in projectiles.iter() {
        let _ = proj;

        let transform = transforms.get(entity).unwrap();
        let proj_pos = transform.translation();

        rapier.intersections_with_shape(
            Vect::new(proj_pos.x, proj_pos.y),
            Rot::default(),
            collider,
            QueryFilter {
                flags: QueryFilterFlags::ONLY_FIXED | QueryFilterFlags::EXCLUDE_SENSORS,
                ..default()
            },
            |hit_entity| {
                proj_collision_events.send(ProjectileCollisionEvent {
                    proj: entity,
                    collision: hit_entity,
                    collision_type: ProjectileCollisionType::SolidTile
                });

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
            |hit_entity| {
                if let Ok(layer) = combat_layers.get(hit_entity) {
                    if layer.is_ally_with(*proj_combat_layer) || immunities.contains(hit_entity) {
                        return true;
                    }

                    let hit_pos = transforms.get(hit_entity).unwrap().translation();
                    let dir = (hit_pos - proj_pos).normalize();

                    hit_events.send(HitEvent {
                        target: hit_entity,
                        damage: strength.power,
                        kb: Vec2::new(dir.x, dir.y)
                    });

                    proj_collision_events.send(ProjectileCollisionEvent {
                        proj: entity,
                        collision: hit_entity,
                        collision_type: ProjectileCollisionType::Entity
                    });
                }

                true
            }
        );
   }
}




pub fn remove_projectiles_on_impact(
    mut commands: Commands,
    entities: Query<Entity>,
    mut events: EventReader<ProjectileCollisionEvent>
) {
    let mut despawn_queue = HashSet::new();

    for collision in events.iter() {
        if !entities.contains(collision.proj) {
            continue;
        }

        despawn_queue.insert(collision.proj);
    }

    for ent in despawn_queue.iter() {
        commands.entity(*ent).despawn();
    }
}