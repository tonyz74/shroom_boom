pub mod state_machine;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use rand::prelude::*;

use crate::{
    common::AnimTimer,
    enemies::{EnemyBundle, Enemy},
    assets::PumpkinEnemyAssets,
    combat::{CombatLayerMask, Health, HurtAbility, KnockbackResistance},
    pathfind::{Pathfinder, PathfinderBundle, util::BoundingBox, walk::WalkPathfinder, RangedPathfinder}
};
use crate::combat::{AttackStrength, ColliderAttackBundle, ProjectileAttack, ProjectileAttackBundle};


#[derive(Component, Copy, Clone, Debug)]
pub struct PumpkinProjectileAttack;

pub struct PumpkinEnemyPlugin;

impl Plugin for PumpkinEnemyPlugin {
    fn build(&self, app: &mut App) {
        let _ = app;
    }
}

#[derive(Component, Default, Debug)]
pub struct PumpkinEnemy;

#[derive(Bundle)]
pub struct PumpkinEnemyBundle {
    #[bundle]
    pub enemy: EnemyBundle,
    pub pumpkin: PumpkinEnemy,
    pub walk: WalkPathfinder,
    pub ranged_pathfinder: RangedPathfinder
}

impl PumpkinEnemyBundle {
    pub fn collider_attack() -> ColliderAttackBundle {
        ColliderAttackBundle {
            combat_layer: CombatLayerMask::ENEMY,
            strength: AttackStrength::new(2),
            ..ColliderAttackBundle::from_size(Vec2::new(36.0, 36.0))
        }
    }

    pub fn spawn(commands: &mut Commands, enemy: Self) {
        commands.spawn(enemy).with_children(|p| {
            p.spawn(Self::collider_attack());
        });
    }

    pub fn from_assets(assets: &Res<PumpkinEnemyAssets>) -> Self {
        PumpkinEnemyBundle {
            enemy: EnemyBundle {
                anim_timer: AnimTimer::from_seconds(assets.anims["IDLE"].speed),

                collider: Collider::cuboid(24.0, 24.0),

                rigid_body: RigidBody::KinematicPositionBased,

                character_controller: KinematicCharacterController {
                    slide: true,
                    snap_to_ground: Some(CharacterLength::Relative(0.2)),
                    offset: CharacterLength::Relative(0.02),
                    filter_flags: QueryFilterFlags::EXCLUDE_SENSORS,
                    ..default()
                },

                state_machine: state_machine::pumpkin_enemy_state_machine(),

                sprite_sheet: SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        custom_size: Some(Vec2::new(48.0, 48.0)),
                        ..default()
                    },
                    texture_atlas: assets.anims["IDLE"].clone().tex,
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                    ..default()
                },

                enemy: Enemy::default(),

                sensor: Sensor,

                path: PathfinderBundle {
                    pathfinder: Pathfinder {
                        speed: thread_rng().gen_range(1.5..2.5),
                        patrol_speed: thread_rng().gen_range(0.8..1.2),
                        bb: BoundingBox::new(24.0, 24.0),
                        ..default()
                    },
                    ..default()
                },

                kb_res: KnockbackResistance::new(1.0),
                combat_layer: CombatLayerMask::ENEMY,

                hurt_ability: HurtAbility::new(0.5, None),

                health: Health::new(10),
            },

            pumpkin: PumpkinEnemy,

            walk: WalkPathfinder {
                jump_speed: 8.0,
                ..default()
            },

            ranged_pathfinder: RangedPathfinder {
                shoot_startup: Timer::from_seconds(0.1, TimerMode::Once),
                shoot_pause: Timer::from_seconds(0.1, TimerMode::Once),
                shoot_cooldown: Timer::from_seconds(1.5, TimerMode::Once),

                max_shoot_angle: 45.0 * (std::f32::consts::PI / 180.0),
                max_shoot_distance: 320.0,

                extra_spawn: |cmd, e| { cmd.entity(e).insert(PumpkinProjectileAttack); },

                projectile: ProjectileAttackBundle {
                    attack: ProjectileAttack {
                        speed: 2.0,
                        ..default()
                    },

                    anim_timer: AnimTimer::from_seconds(0.1),

                    sprite_sheet: SpriteSheetBundle {
                        sprite: TextureAtlasSprite {
                            custom_size: Some(Vec2::new(16.0, 16.0)),
                            ..default()
                        },
                        texture_atlas: assets.anims["IDLE"].clone().tex,
                        ..default()
                    },

                    strength: AttackStrength::new(5),

                    combat_layer: CombatLayerMask::ENEMY,

                    ..ProjectileAttackBundle::from_size(Vec2::new(16.0, 16.0))
                },

                ..default()
            },
        }
    }
}
