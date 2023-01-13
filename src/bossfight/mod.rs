pub mod state_machine;

use bevy::prelude::*;
use bevy::time::FixedTimestep;
use seldom_state::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::assets::BossAssets;
use crate::bossfight::state_machine::boss_state_machine;
use crate::coin::drops::CoinHolder;
use crate::combat::{AttackStrength, ColliderAttackBundle, CombatLayerMask, Health, HurtAbility, KnockbackResistance};
use crate::common::{AnimTimer, PHYSICS_STEP_DELTA, PHYSICS_STEPS_PER_SEC};
use crate::enemies::Enemy;
use crate::entity_states::*;
use crate::state::GameState;

#[derive(Component, Copy, Clone, Reflect)]
pub enum BossStage {
    Waiting,
    SummonEasy,
    VulnerableEasy,
    SummonMedium,
    VulnerableMedium,
    SummonHard,
    VulnerableHard,
    Enraged,
}

impl Default for BossStage {
    fn default() -> Self {
        Self::Waiting
    }
}

#[derive(Component, Copy, Clone, Default, Reflect)]
pub struct Boss {
    pub stage: BossStage,
    pub grounded: bool
}

#[derive(Bundle, Clone)]
pub struct BossBundle {
    pub boss: Boss,

    pub enemy: Enemy,
    pub sensor: Sensor,
    pub anim_timer: AnimTimer,
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub state_machine: StateMachine,
    pub character_controller: KinematicCharacterController,

    pub hurt_ability: HurtAbility,

    pub health: Health,
    pub kb_res: KnockbackResistance,
    pub combat_layer: CombatLayerMask,

    pub coins: CoinHolder,

    #[bundle]
    pub sprite_sheet: SpriteSheetBundle,
}

impl BossBundle {
    pub fn collider_attack() -> ColliderAttackBundle {
        ColliderAttackBundle {
            combat_layer: CombatLayerMask::ENEMY,
            strength: AttackStrength::new(4),
            ..ColliderAttackBundle::from_size(Vec2::new(256.0, 512.0))
        }
    }

    pub fn spawn(commands: &mut Commands, boss: BossBundle) {
        commands.spawn(boss).with_children(|p| {
            p.spawn(Self::collider_attack());
        });
    }

    pub fn from_assets(assets: &BossAssets) -> Self {
        let anim = &assets.anims["WAIT"];

        Self {
            boss: Boss {
                stage: BossStage::Waiting,
                ..default()
            },

            enemy: Enemy::default(),

            sensor: Sensor,

            anim_timer: AnimTimer::from_seconds(anim.speed),

            collider: Collider::cuboid(128.0, 256.0),

            rigid_body: RigidBody::KinematicPositionBased,

            state_machine: boss_state_machine(),

            character_controller: KinematicCharacterController {
                filter_flags: QueryFilterFlags::EXCLUDE_SENSORS,
                ..default()
            },

            hurt_ability: HurtAbility::new(0.3, Some(0.3)),

            health: Health::new(120),

            kb_res: KnockbackResistance::new(0.1),

            combat_layer: CombatLayerMask::ENEMY,

            coins: CoinHolder {
                total_value: 120
            },

            sprite_sheet: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::new(512.0, 512.0)),
                    ..default()
                },

                texture_atlas: anim.tex.clone(),

                ..default()
            }
        }
    }
}





pub struct BossPlugin;

impl Plugin for BossPlugin {
    fn build(&self, app: &mut App) {
       app
           .register_type::<Boss>()
           .add_system_set(
               SystemSet::on_update(GameState::Gameplay)
                   .with_system(boss_set_grounded)
                   .with_system(boss_got_hurt)
           )
           .add_system_set(
               SystemSet::on_update(GameState::Gameplay)
                   .with_run_criteria(FixedTimestep::steps_per_second(PHYSICS_STEPS_PER_SEC))
                   .with_system(boss_fall)
            );
    }
}



pub fn boss_fall(
    mut q: Query<(&mut Enemy, &Boss)>
) {
    for (mut enemy, boss) in q.iter_mut() {
        if boss.grounded {
            continue;
        }

        enemy.vel.y += -20.0 * PHYSICS_STEP_DELTA;

        if enemy.vel.y <= -20.0 {
            enemy.vel.y = -20.0;
        }
    }
}

pub fn boss_got_hurt(
    mut q: Query<&mut HurtAbility, With<Boss>>
) {
    for mut hurt in q.iter_mut() {
        hurt.hit_event = None;
    }
}

pub fn boss_set_grounded(
    mut q: Query<(&mut Boss, &KinematicCharacterControllerOutput)>
) {
    for (mut boss, cc_out) in q.iter_mut() {
        boss.grounded = cc_out.grounded;
    }
}