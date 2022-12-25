use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::attack::{AttackStrength, CombatLayerMask};
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
                filter_flags: QueryFilterFlags::EXCLUDE_SENSORS,
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
    mut proj: Query<(
        Entity,
        &KinematicCharacterControllerOutput
    ), With<ProjectileAttack>>,
    rigid_bodies: Query<&RigidBody>,
    combat_masks: Query<&CombatLayerMask>,

) {
    for (entity, cc_out) in proj.iter() {
        for collision in &cc_out.collisions {
            let rb = rigid_bodies.get(collision.entity).unwrap();

            if *rb == RigidBody::Fixed {
                println!("woooooohoo");
                commands.entity(collision.entity).despawn();
            }
        }
    }
}
