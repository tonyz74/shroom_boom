use bevy::prelude::*;
use bevy_debug_text_overlay::screen_print;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::InputManagerBundle;
use seldom_state::prelude::StateMachine;
use crate::anim::Animator;

use crate::{
    state::GameState,
    input::InputAction,
    assets::PlayerAssets,
    combat::ColliderAttackBundle
};

pub mod consts;
pub mod anim;
pub mod logic;
pub mod triggers;
pub mod state_machine;
pub mod abilities;
pub mod ammo;
pub mod skill;

use abilities::dash::DashAbility;
use crate::coin::drops::CoinHolder;
use crate::coin::pickup::CoinCollector;
use crate::combat::{AttackStrength, ColliderAttack, CombatLayerMask, Health, HurtAbility, Immunity};
use crate::input::PlayerControls;
use crate::level::consts::SOLIDS_INTERACTION_GROUP;
use crate::player::abilities::slash::SlashAbility;
use crate::player::abilities::jump::JumpAbility;
use crate::player::abilities::shoot::ShootAbility;
use crate::player::ammo::Ammo;
use crate::player::consts::HEALTH_LEVELS;
use crate::player::skill::{PlayerSkillLevels, upgrade_player_from_skills};
use crate::util::Facing;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub facing: Facing,

    pub slash: SlashAbility,
    pub dash: DashAbility,
    pub jump: JumpAbility,
    pub shoot: ShootAbility,
    pub hurt: HurtAbility,

    pub skill_levels: PlayerSkillLevels,

    pub character_controller: KinematicCharacterController,
    pub rigid_body: RigidBody,
    pub sensor: Sensor,
    pub collider: Collider,
    pub state_machine: StateMachine,
    pub anim: Animator,

    pub ammo: Ammo,
    pub immunity: Immunity,
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

        app
            // .add_system(player_print_health)
            .add_system(player_pos)
            .add_system(upgrade_player_from_skills);

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
    exists: Query<&Player>,
    controls: Res<PlayerControls>,
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

            anim: Animator::new(anim.clone()),

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
            facing: Facing::default(),

            sensor: Sensor,

            dash: DashAbility::default(),
            slash: SlashAbility::default(),
            jump: JumpAbility::default(),
            shoot: ShootAbility::default(),
            hurt: HurtAbility::new(0.4, Some(0.3)),

            skill_levels: PlayerSkillLevels::default(),

            coin_holder: CoinHolder { total_value: 0 },
            coin_collector: CoinCollector,

            input: InputAction::input_manager_bundle(&controls),

            state_machine: state_machine::player_state_machine(),

            combat_layer: CombatLayerMask::PLAYER,
            health: Health::new(HEALTH_LEVELS[0]),

            immunity: Immunity::default()
        }
    ).with_children(|p| {
        p.spawn(ColliderAttackBundle {
            strength: AttackStrength::new(0),
            combat_layer: CombatLayerMask::PLAYER,
            attack: ColliderAttack { enabled: false },
            ..ColliderAttackBundle::from_size(Vec2::new(64.0, 52.0))
        });
    });
}

pub fn player_pos(
    p: Query<&GlobalTransform, With<Player>>
) {
    for tf in p.iter() {
        // screen_print!("{:?}", tf.translation());
    }
}
