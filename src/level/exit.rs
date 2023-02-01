use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    level::{
        util,
        LevelRegion, FinishedTransitioning,
        transition::{LevelTransition, TransitionEffect}
    },
    state::GameState,
    player::Player,
};
use crate::level::{coord, LevelInfo};
use crate::level::consts::SCALE_FACTOR;

#[derive(Component, Default)]
pub struct ExitTileMarker;

#[derive(Bundle, LdtkEntity)]
pub struct ExitTileBundle {
    marker: ExitTileMarker,
    #[from_entity_instance]
    instance: EntityInstance
}

#[derive(Component)]
pub struct LevelExit {
    pub link: i32,
    pub entry_point_id: i32
}

pub fn register_exit_entity(app: &mut App) {
    app
        .register_ldtk_entity::<ExitTileBundle>("Exit")
        .add_system_set(
            SystemSet::on_update(GameState::LevelTransition)
                .with_system(add_exit_entities)
        )
        .add_system_set(
            SystemSet::on_update(GameState::Gameplay)
                .with_system(on_player_collide_with_exit)
                .with_system(on_player_out_of_bounds)
        );
}

fn add_exit_entities(
    mut commands: Commands,
    q: Query<&EntityInstance, Added<ExitTileMarker>>,
    lvl_info: Res<LevelInfo>
) {
    for inst in q.iter() {
        let link = util::val_expect_i32(&inst.field_instances[0].value).unwrap();
        let entry_point_id = util::val_expect_i32(&inst.field_instances[1].value).unwrap();

        let tl = coord::grid_coord_to_translation(inst.grid, lvl_info.grid_size.as_ivec2());

        commands.spawn((
            Sensor,
            RigidBody::Fixed,
            Collider::cuboid(inst.width as f32 / 2., inst.height as f32 / 2.),
            TransformBundle::from_transform(
                Transform::default()
                    .with_translation(Vec3::new(tl.x, tl.y, 0.0))
                    .with_scale(Vec3::new(SCALE_FACTOR, SCALE_FACTOR, 1.0))
            ),

            LevelExit {
                link, entry_point_id
            }
        ));
    }
}

fn on_player_collide_with_exit(
    player_q: Query<Entity, With<Player>>,
    exit_q: Query<(Entity, &LevelExit)>,
    rapier: Res<RapierContext>,
    mut sel: ResMut<LevelTransition>
) {
    if player_q.is_empty() {
        return;
    }

    let p = player_q.single();
    for (exit, info) in exit_q.iter() {
        if rapier.intersection_pair(exit, p).is_some() {
            *sel = LevelTransition {
                next: info.link,
                entry_point_id: info.entry_point_id,
                transition_effect: TransitionEffect::default()
            };
        }
    }
}

fn on_player_out_of_bounds(
    player: Query<(&GlobalTransform, Entity), (With<Player>, With<FinishedTransitioning>)>,
    region: Query<Entity, With<LevelRegion>>,
    mut state: ResMut<State<GameState>>,
    rapier: Res<RapierContext>
) {
    let current = state.current();
    if *current != GameState::Gameplay {
        return;
    }

    for (pos, player) in player.iter() {
        for region in region.iter() {
            if rapier.intersection_pair(player, region).is_none() {
                info!("Player exiting the level at {:?}", pos.translation());
                state.set(GameState::LevelTransition).unwrap();
            }
        }
    }
}
