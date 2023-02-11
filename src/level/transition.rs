use bevy::prelude::*;
use bevy_debug_text_overlay::screen_print;
use bevy_ecs_ldtk::prelude::*;
use crate::camera::GameCamera;
use crate::coin::coin::Coin;
use crate::combat::{ExplosionAttack, ProjectileAttack};
use crate::enemies::Enemy;
use crate::level::{FinishedTransitioning, exit::LevelExit, LevelInfo};
use crate::level::consts::TILE_SIZE;
use crate::level::tutorial::HelpText;
use crate::state::GameState;
use crate::player::Player;
use crate::shop::Shop;


#[derive(Resource, Default)]
pub struct TransitionCleanupEvent {
    pub new_level: String
}

#[derive(Resource, Default)]
pub struct TransitionSetupEvent {
    pub new_level: String
}


#[derive(Resource, Default)]
pub struct LevelTransition {
    pub next: String,
    pub transition_effect: TransitionEffect
}

#[derive(Clone, Resource)]
pub enum TransitionEffect {
    Fade(FadeTransition),
}

impl Default for TransitionEffect {
    fn default() -> Self {
       Self::Fade(FadeTransition::default())
    }
}

#[derive(Resource, Clone)]
pub struct FadeTransition {
    pub mask: Option<Entity>,
    pub fade_in: Timer,
    pub fade_out: Timer,
    pub fade_pause: Timer,
    pub fade_color: Color
}

impl Default for FadeTransition {
    fn default() -> Self {
        Self {
            mask: None,
            fade_in: Timer::from_seconds(0.2, TimerMode::Once),
            fade_out: Timer::from_seconds(0.2, TimerMode::Once),
            fade_pause: Timer::from_seconds(0.5, TimerMode::Once),
            fade_color: Color::BLACK
        }
    }
}

/// SYSTEMS

pub fn register_transition_systems(app: &mut App) {
    app
        .add_event::<TransitionCleanupEvent>()
        .add_event::<TransitionSetupEvent>()
        .init_resource::<LevelTransition>()
        .add_system_set(
            SystemSet::on_update(GameState::LevelTransition)
                .with_system(transition_update_effect)
                .with_system(transition_on_update)
                .with_system(transition_cleanup_old)
                .with_system(transition_setup_new)
        )
        .add_system_set(
            SystemSet::on_enter(GameState::LevelTransition)
                .with_system(transition_on_start)
        )
        .add_system_set(
            SystemSet::on_exit(GameState::LevelTransition)
                .with_system(exit_transition)
        );
}

pub fn transition_cleanup_old(
    mut commands: Commands,
    exits: Query<Entity, With<LevelExit>>,
    mut player: Query<(Entity, &mut Player)>,
    enemies: Query<Entity, With<Enemy>>,
    shopkeepers: Query<Entity, With<Shop>>,
    projectiles: Query<Entity, With<ProjectileAttack>>,
    misc: Query<Entity, Or<(With<ProjectileAttack>, With<ExplosionAttack>, With<Coin>, With<HelpText>)>>,

    mut transition_cleanup_event: EventReader<TransitionCleanupEvent>,
    mut setup: EventWriter<TransitionSetupEvent>,

    sel: Res<LevelSelection>,
    trans: Res<LevelTransition>
) {
    if transition_cleanup_event.is_empty() {
        return;
    }

    for ev in transition_cleanup_event.iter() {
        if LevelSelection::Identifier(trans.next.clone()) != sel.clone() {
            for exit in exits.iter() {
                commands.entity(exit).despawn();
            }

            for (entity, mut player) in player.iter_mut() {
                commands.entity(entity).remove::<FinishedTransitioning>();
                player.vel = Vec2::ZERO;
            }

            for enemy in enemies.iter() {
                commands.entity(enemy).despawn_recursive();
            }

            for shopkeeper in shopkeepers.iter() {
                commands.entity(shopkeeper).despawn_recursive();
            }

            for misc in misc.iter() {
                commands.entity(misc).despawn_recursive();
            }

            for proj in projectiles.iter() {
                commands.entity(proj).despawn_recursive();
            }
        }

        setup.send(TransitionSetupEvent { new_level: ev.new_level.clone() });
    }
}

pub fn transition_setup_new(
    mut setup: EventReader<TransitionSetupEvent>,
    mut sel: ResMut<LevelSelection>,
    levels: Query<&Handle<LdtkAsset>>,
    assets: Res<Assets<LdtkAsset>>,

    mut lvl_info: ResMut<LevelInfo>
) {
    if setup.is_empty() {
        return;
    }

    for ev in setup.iter() {
        *sel = LevelSelection::Identifier(ev.new_level.clone());
    }

    let lvl = assets
        .get(levels.single())
        .unwrap()
        .get_level(&sel)
        .unwrap();

    lvl_info.grid_size = IVec2::new(lvl.px_wid, lvl.px_hei).as_vec2() / TILE_SIZE;
    println!("Set new level");
}

pub fn transition_on_start(
    mut commands: Commands,
    mut trans: ResMut<LevelTransition>,
) {
    match &mut trans.transition_effect {
        TransitionEffect::Fade(fade) => {
            fade.fade_in.reset();
            fade.fade_out.reset();
            fade.fade_pause.reset();

            fade.mask = Some(commands.spawn(
                SpriteBundle {
                    sprite: Sprite {
                        color: fade.fade_color,
                        custom_size: Some(Vec2::new(16000.0, 16000.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 998.0)),
                    ..default()
                }
            ).id());
        }
    }
}

pub fn exit_transition(mut commands: Commands, transition: Res<LevelTransition>) {
    match &transition.transition_effect {
        TransitionEffect::Fade(trans) => {
            commands.entity(trans.mask.unwrap()).despawn();
        }
    }
}

pub fn transition_on_update(
    time: Res<Time>,
    mut trans: ResMut<LevelTransition>,
    mut sel: ResMut<LevelSelection>,
    mut state: ResMut<State<GameState>>,
    mut events: EventWriter<TransitionCleanupEvent>
) {
    let dt = time.delta();
    let next_level = trans.next.clone();

    match &mut trans.transition_effect {
        TransitionEffect::Fade(fade) => {
            if next_level == "None" {
                trans.next = String::from("Init");
                state.push(GameState::GameWonMenu).unwrap();
                return;
            }

            fade.fade_in.tick(dt);
            if fade.fade_in.just_finished() {
                events.send(TransitionCleanupEvent { new_level: next_level.clone() });
            }

            if fade.fade_in.finished() && !fade.fade_in.just_finished() {
                fade.fade_pause.tick(dt);

                if fade.fade_pause.finished() {
                    fade.fade_out.tick(dt);

                    if fade.fade_out.just_finished() {
                        state.set(GameState::Gameplay).unwrap();
                    }
                }
            }
        }
    }
}

pub fn transition_update_effect(
    mut trans: ResMut<LevelTransition>,
    camera: Query<&GlobalTransform, With<GameCamera>>,
    mut pos: Query<(&mut Transform, &mut Sprite)>,
) {
    if camera.is_empty() {
        return;
    }

    let cam_pos = camera.single().translation();

    match &mut trans.transition_effect {
        TransitionEffect::Fade(fade) => {
            let (mut transform, mut spr) = pos.get_mut(fade.mask.unwrap()).unwrap();

            transform.translation.x = cam_pos.x;
            transform.translation.y = cam_pos.y;

            let percent = if !fade.fade_in.finished() {
                fade.fade_in.percent()
            } else {
                fade.fade_out.percent_left()
            };

            spr.color = Color::rgba(
                fade.fade_color.r(),
                fade.fade_color.g(),
                fade.fade_color.b(),
                percent
            );
        }
    };
}