mod state_machine;
mod summon;
mod vulnerable;
mod enraged;
mod stage;
mod abilities;

use bevy::prelude::*;
use bevy_debug_text_overlay::screen_print;
use seldom_state::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::assets::BossAssets;
use crate::bossfight::enraged::{EnragedAttackMove, register_boss_enraged};
use crate::bossfight::stage::BossStage;
use crate::bossfight::state_machine::{boss_state_machine, register_boss_state_machine};
use crate::bossfight::summon::register_boss_summon;
use crate::bossfight::vulnerable::register_boss_vulnerable;
use crate::coin::drops::CoinHolder;
use crate::combat::{AttackStrength, ColliderAttackBundle, CombatLayerMask, Health, HurtAbility};
use crate::common::AnimTimer;
use crate::enemies::Enemy;
use crate::entity_states::*;
use crate::state::GameState;
use enraged::ATTACK_SEQUENCE;
use crate::bossfight::abilities::{BoomAbility, register_boss_abilities, RestAbility};


#[derive(Copy, Clone, Debug, Component, Reflect)]
pub struct Airborne;

#[derive(Component, Clone, Reflect)]
pub struct Boss {
    pub grounded: bool,
    pub vulnerability_timer: Timer,
    pub move_index: usize
}

impl Default for Boss {
    fn default() -> Self {
        Self {
            grounded: false,
            vulnerability_timer: Timer::from_seconds(8.0, TimerMode::Once),
            move_index: 0
        }
    }
}

impl Boss {
    pub fn current_move(&self) -> EnragedAttackMove {
        ATTACK_SEQUENCE[self.move_index]
    }
}

#[derive(Bundle, Clone)]
pub struct BossBundle {
    pub boss: Boss,
    pub stage: BossStage,

    pub enemy: Enemy,
    pub sensor: Sensor,
    pub anim_timer: AnimTimer,
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub state_machine: StateMachine,
    pub character_controller: KinematicCharacterController,

    pub rest: RestAbility,
    pub hurt: HurtAbility,
    pub boom: BoomAbility,

    pub health: Health,
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

    pub fn spawn(commands: &mut Commands, boss: BossBundle) -> Entity {
        commands.spawn(boss).with_children(|p| {
            p.spawn(Self::collider_attack());
        }).id()
    }

    pub fn from_assets(assets: &BossAssets) -> Self {
        let anim = &assets.anims["WAIT"];

        Self {
            boss: Boss::default(),

            stage: BossStage::Waiting,

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

            rest: RestAbility::default(),
            hurt: HurtAbility::new(0.3, Some(0.3)),
            boom: BoomAbility::default(),

            health: Health::new(200),

            combat_layer: CombatLayerMask::ENEMY,

            coins: CoinHolder {
                total_value: 120
            },

            sprite_sheet: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::new(256.0, 512.0)),
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
        register_boss_state_machine(app);
        register_boss_summon(app);
        register_boss_vulnerable(app);
        register_boss_enraged(app);
        register_boss_abilities(app);

        app
            .register_type::<Boss>()
            .register_type::<BossStage>()
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay)
                    .with_system(boss_got_hurt)
                    .with_system(boss_start_idling)
                    .with_system(print_stage)
            );
    }
}


pub fn print_stage(
    q: Query<(&Boss, &BossStage)>
) {
    for (boss, stage) in q.iter() {
        screen_print!(
            "boss stage: {:?}, current move: {:?}",
            stage, boss.current_move()
        );
    }
}


pub fn boss_start_idling(mut q: Query<&mut Enemy, (With<Boss>, Added<Idle>)>) {
    for mut enemy in q.iter_mut() {
        enemy.vel = Vec2::ZERO;
    }
}


pub fn boss_got_hurt(
    mut q: Query<(&mut HurtAbility, &Health, &mut BossStage), Added<Hurt>>
) {
    for (mut hurt, health, mut stage) in q.iter_mut() {

        if hurt.hit_event.is_some() {
            if stage.clone() == BossStage::Waiting {
                *stage = BossStage::from_health(health.hp);
            }
        }

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