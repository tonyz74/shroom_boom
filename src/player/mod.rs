use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::InputManagerBundle;
use seldom_state::prelude::StateMachine;

use crate::{
    state::GameState,
    input::InputAction,
    assets::PlayerAssets,
    common::AnimTimer,
    combat::ColliderAttackBundle
};

pub mod consts;
pub mod anim;
pub mod logic;
pub mod triggers;
pub mod state_machine;
pub mod abilities;
pub mod ammo;

use abilities::dash::DashAbility;
use crate::coin::drops::CoinHolder;
use crate::coin::pickup::CoinCollector;
use crate::combat::{AttackStrength, ColliderAttack, CombatLayerMask, Health, HurtAbility};
use crate::level::consts::SOLIDS_INTERACTION_GROUP;
use crate::player::abilities::slash::SlashAbility;
use crate::player::abilities::jump::JumpAbility;
use crate::player::abilities::shoot::ShootAbility;
use crate::player::ammo::Ammo;
use crate::util::Facing;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,

    pub slash: SlashAbility,
    pub dash: DashAbility,
    pub jump: JumpAbility,
    pub shoot: ShootAbility,
    pub hurt: HurtAbility,

    pub character_controller: KinematicCharacterController,
    pub rigid_body: RigidBody,
    pub sensor: Sensor,
    pub collider: Collider,
    pub state_machine: StateMachine,
    pub anim_timer: AnimTimer,

    pub ammo: Ammo,

    pub combat_layer: CombatLayerMask,
    pub health: Health,

    pub coin_holder: CoinHolder,
    pub coin_collector: CoinCollector,

    #[bundle]
    pub input: InputManagerBundle<InputAction>,
    #[bundle]
    pub sprite_sheet: SpriteSheetBundle
}

#[derive(Component, Default)]
pub struct Player {
    pub vel: Vec2,
    pub grounded: bool,
    pub facing: Facing
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::LevelTransition)
                .with_system(setup_player)
        );

        app.add_system(player_print_health);

        anim::player_setup_anim(app);
        logic::player_setup_logic(app);
        triggers::player_setup_triggers(app);
        ammo::register_ammo(app);
    }
}

#[derive(Component)]
pub struct FollowMarker;

fn setup_player(
    mut commands: Commands,
    assets: Res<PlayerAssets>,
    exists: Query<&Player>
) {
    if !exists.is_empty() {
        return;
    }

    use consts::{PLAYER_COLLIDER_CAPSULE, PLAYER_SIZE_PX};
    let anim = &assets.anims["IDLE"];

    commands.spawn(
        PlayerBundle {
            sprite_sheet: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(PLAYER_SIZE_PX),
                    ..default()
                },
                texture_atlas: anim.tex.clone(),
                ..default()
            },

            anim_timer: AnimTimer::from_seconds(anim.speed),

            ammo: Ammo::default(),

            collider: Collider::capsule(
                PLAYER_COLLIDER_CAPSULE.segment.a.into(),
                PLAYER_COLLIDER_CAPSULE.segment.b.into(),
                PLAYER_COLLIDER_CAPSULE.radius
            ),

            rigid_body: RigidBody::KinematicPositionBased,

            character_controller: KinematicCharacterController {
                slide: true,
                snap_to_ground: Some(CharacterLength::Relative(0.2)),
                offset: CharacterLength::Relative(0.02),
                filter_flags: QueryFilterFlags::EXCLUDE_SENSORS,
                filter_groups: Some(SOLIDS_INTERACTION_GROUP),
                ..default()
            },

            player: Player::default(),

            sensor: Sensor,

            dash: DashAbility::default(),
            slash: SlashAbility::default(),
            jump: JumpAbility::default(),
            shoot: ShootAbility::default(),
            hurt: HurtAbility::new(0.3, Some(0.3)),

            coin_holder: CoinHolder::default(),
            coin_collector: CoinCollector,

            input: InputAction::input_manager_bundle(),

            state_machine: state_machine::player_state_machine(),

            combat_layer: CombatLayerMask::PLAYER,
            health: Health::new(100000)
        }
    ).with_children(|p| {
        p.spawn(ColliderAttackBundle {
            strength: AttackStrength::new(80),
            combat_layer: CombatLayerMask::PLAYER,
            attack: ColliderAttack { enabled: false },
            ..ColliderAttackBundle::from_size(Vec2::new(32.0, 40.0))
        });
    });
}

pub fn player_print_health(p: Query<&Health, Changed<Health>>) {
    for hp in p.iter() {
        println!("hp changed: {:?}", hp);
    }
}
