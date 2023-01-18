mod state_machine;
mod summon;
mod vulnerable;
mod enraged;
mod stage;
mod abilities;
mod config;
mod consts;

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
use crate::combat::{AttackStrength, ColliderAttack, ColliderAttackBundle, CombatLayerMask, Health, HurtAbility, Immunity, KnockbackModifier};
use crate::common::AnimTimer;
use crate::enemies::Enemy;
use crate::entity_states::*;
use crate::state::GameState;
use enraged::ATTACK_SEQUENCE;
use crate::bossfight::abilities::{BoomAbility, RelocateAbility, register_boss_abilities, RestAbility, ChargeAbility, LeapAbility, HoverAbility, SlamAbility, TakeoffAbility};

pub use crate::bossfight::config::BossConfig;
use crate::bossfight::consts::{BOSS_FULL_SIZE, BOSS_HALF_SIZE, BOSS_HEALTH};
use crate::util::Facing;


#[derive(Component, Clone, Reflect)]
pub struct Boss {
    pub grounded: bool,
    pub vulnerability_timer: Timer,
    pub move_index: usize,
    pub facing: Facing
}

impl Default for Boss {
    fn default() -> Self {
        Self {
            grounded: false,
            vulnerability_timer: Timer::from_seconds(8.0, TimerMode::Once),
            move_index: 0,
            facing: Facing::Right
        }
    }
}

impl Boss {
    pub fn current_move(&self) -> EnragedAttackMove {
        ATTACK_SEQUENCE[self.move_index]
    }
    pub fn next_move(&self) -> EnragedAttackMove {
        let next = (self.move_index + 1) % ATTACK_SEQUENCE.len();
        ATTACK_SEQUENCE[next]
    }
}

#[derive(Bundle, Clone)]
pub struct BossBundle {
    pub boss: Boss,
    pub stage: BossStage,
    pub config: BossConfig,

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
    pub slam: SlamAbility,
    pub charge: ChargeAbility,
    pub leap: LeapAbility,
    pub hover: HoverAbility,
    pub takeoff: TakeoffAbility,
    pub relocate: RelocateAbility,

    pub health: Health,
    pub immunity: Immunity,
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
            knockback: KnockbackModifier::new(|kb| {
                let x_dir = Vec2::new(kb.x, 0.0).normalize().x;
                kb + Vec2::new(x_dir * 4.0, 6.0)
            }),
            attack: ColliderAttack { enabled: false },
            ..ColliderAttackBundle::from_size(BOSS_FULL_SIZE)
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
            immunity: Immunity::default(),
            config: BossConfig::default(),
            boss: Boss::default(),
            stage: BossStage::Waiting,
            enemy: Enemy::default(),
            sensor: Sensor,
            anim_timer: AnimTimer::from_seconds(anim.speed),
            collider: Collider::cuboid(BOSS_HALF_SIZE.x, BOSS_HALF_SIZE.y),
            rigid_body: RigidBody::KinematicPositionBased,
            state_machine: boss_state_machine(),

            character_controller: KinematicCharacterController {
                filter_flags: QueryFilterFlags::EXCLUDE_SENSORS,
                snap_to_ground: None,
                offset: CharacterLength::Relative(0.0),
                ..default()
            },

            rest: RestAbility::default(),
            hurt: HurtAbility::new(0.3, Some(0.3)),
            boom: BoomAbility::default(),
            charge: ChargeAbility::default(),
            leap: LeapAbility::default(),
            hover: HoverAbility::default(),
            slam: SlamAbility::default(),
            takeoff: TakeoffAbility::default(),
            relocate: RelocateAbility::default(),

            health: Health::new(101),

            combat_layer: CombatLayerMask::ENEMY,

            coins: CoinHolder {
                total_value: 120
            },

            sprite_sheet: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(BOSS_FULL_SIZE),
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
            .register_type::<BossConfig>()
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