mod state_machine;
mod summon;
mod vulnerable;
mod enraged;
pub mod stage;
mod abilities;
mod config;
pub mod consts;
mod util;
mod anim;

use bevy::prelude::*;
use bevy_debug_text_overlay::screen_print;

use seldom_state::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::assets::BossAssets;
use crate::bossfight::enraged::{EnragedAttackMove, register_boss_enraged};
use crate::bossfight::stage::BossStage;
use crate::bossfight::state_machine::{boss_state_machine, register_boss_state_machine};
use crate::bossfight::summon::{register_boss_summon, SummonAbility};
use crate::bossfight::vulnerable::register_boss_vulnerable;
use crate::coin::drops::CoinHolder;
use crate::combat::{AttackStrength, ColliderAttack, ColliderAttackBundle, CombatLayerMask, Health, HurtAbility, Immunity, KnockbackModifier};
use crate::enemies::Enemy;
use crate::entity_states::*;
use crate::state::GameState;
use enraged::ATTACK_SEQUENCE;
use crate::bossfight::abilities::{BoomAbility, RelocateAbility, register_boss_abilities, RestAbility, ChargeAbility, LeapAbility, HoverAbility, SlamAbility, TakeoffAbility};

pub use crate::bossfight::config::BossConfig;
use crate::bossfight::consts::{BOSS_FULL_SIZE, BOSS_HALF_SIZE, BOSS_HEALTH};
use crate::util::Facing;
use crate::anim::Animator;
use crate::anim::map::AnimationMap;
use crate::bossfight::anim::register_boss_animations;


#[derive(Component, Clone, Reflect)]
pub struct Boss {
    pub vulnerability_timer: Timer,
    pub move_index: usize,
}

impl Default for Boss {
    fn default() -> Self {
        Self {
            vulnerability_timer: Timer::from_seconds(12.0, TimerMode::Once),
            move_index: 0,
        }
    }
}

impl Boss {
    pub fn current_move(&self) -> EnragedAttackMove {
        ATTACK_SEQUENCE[self.move_index]
    }

    pub fn previous_move(&self) -> EnragedAttackMove {
        let i = if self.move_index == 0 {
            ATTACK_SEQUENCE.len() - 1
        } else {
            self.move_index - 1
        };

        ATTACK_SEQUENCE[i]
    }

    pub fn next_move(&self) -> EnragedAttackMove {
        let next = (self.move_index + 1) % ATTACK_SEQUENCE.len();
        ATTACK_SEQUENCE[next]
    }
}

#[derive(Bundle, Clone)]
pub struct BossBundle {
    pub boss: Boss,
    pub facing: Facing,
    pub stage: BossStage,
    pub config: BossConfig,

    pub anim: Animator,
    pub anim_map: AnimationMap,

    pub enemy: Enemy,
    pub sensor: Sensor,
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub state_machine: StateMachine,
    pub character_controller: KinematicCharacterController,

    pub summon: SummonAbility,
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
            strength: AttackStrength::new(12),
            knockback: KnockbackModifier::new(|kb| {
                let x_dir = Vec2::new(kb.x, 0.0).normalize().x;
                kb + Vec2::new(x_dir * 4.0, 9.0)
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
        let anim = &assets.anims["IDLE"];

        Self {
            anim: Animator::new(anim.clone()),
            anim_map: assets.anims.clone(),

            immunity: Immunity::default(),
            config: BossConfig::default(),
            facing: Facing::default(),
            boss: Boss::default(),
            stage: BossStage::Waiting,
            enemy: Enemy::default(),
            sensor: Sensor,
            collider: Collider::cuboid(BOSS_HALF_SIZE.x, BOSS_HALF_SIZE.y),
            rigid_body: RigidBody::KinematicPositionBased,
            state_machine: boss_state_machine(),

            character_controller: KinematicCharacterController {
                filter_flags: QueryFilterFlags::EXCLUDE_SENSORS,
                snap_to_ground: None,
                offset: CharacterLength::Relative(0.0),
                ..default()
            },

            summon: SummonAbility::default(),
            rest: RestAbility::default(),
            hurt: HurtAbility::new(0.5, Some(0.5)),
            boom: BoomAbility::default(),
            charge: ChargeAbility::default(),
            leap: LeapAbility::default(),
            hover: HoverAbility::default(),
            slam: SlamAbility::default(),
            takeoff: TakeoffAbility::default(),
            relocate: RelocateAbility::default(),

            health: Health::new(BOSS_HEALTH),

            combat_layer: CombatLayerMask::ENEMY,

            coins: CoinHolder {
                total_value: 1000
            },

            sprite_sheet: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::new(528.0, 528.0)),
                    ..default()
                },

                texture_atlas: anim.tex.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 1.0),

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
        register_boss_animations(app);

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
    colliders: Query<&ColliderAttack>,
    q: Query<(&Children, &Boss, &BossStage, &Facing)>
) {
    for (children, boss, stage, facing) in q.iter() {
        let mut c = None;
        for child in children {
            if let Ok(atk) = colliders.get(*child) {
                c = Some(atk);
            }
        }

        // screen_print!(
        //     "boss stage: {:?}, current move: {:?}, facing: {:?}, c: {:?}",
        //     stage, boss.current_move(), facing, c
        // );
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