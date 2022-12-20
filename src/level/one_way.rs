use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use bevy_rapier2d::prelude::*;
use std::collections::HashMap;

use crate::{
    level::consts::{
        ALL_PLATFORMS_INTERACTION_GROUP,
        SOLIDS_INTERACTION_GROUP,
        ONE_WAY_PLATFORMS_COLLISION_GROUP
    },
    state::GameState,
    player::{
        Player,
        state_machine as s,
        consts::{PLAYER_TERMINAL_VELOCITY, PLAYER_SIZE_PX}
    }
};

#[derive(Default, Component)]
pub struct OneWayTileSpawnMarker;

#[derive(Bundle, LdtkIntCell)]
pub struct OneWayTileBundle {
    marker: OneWayTileSpawnMarker
}

#[derive(Component)]
pub struct OneWayTile;

pub fn register_one_way_tile(app: &mut App) {
    app
        .register_ldtk_int_cell_for_layer::<OneWayTileBundle>("Tiles", 2)
        .add_system_set(
            SystemSet::on_update(GameState::LevelTransition)
                .with_system(add_one_way_tiles)
        )
        .add_system_set(
            SystemSet::on_update(GameState::Gameplay)
                .with_system(enable_one_way_colliders)
                .with_system(disable_one_way_colliders)
        );
}

#[derive(Default)]
struct PlatformChunk {
    start: i32,
    end: i32
}

pub fn add_one_way_tiles(
    mut commands: Commands,
    colliders: Query<(&Parent, &GridCoords), Added<OneWayTileSpawnMarker>>
) {
    if colliders.is_empty() {
       return;
    }

    let mut common_parent = Entity::from_raw(0);
    let mut platforms_by_y = HashMap::new();

    for (parent, coords) in colliders.iter() {
        platforms_by_y
            .entry(coords.y)
            .or_insert(vec![])
            .push(coords.x);

        common_parent = parent.get();
    }

    for (y_level, vec) in platforms_by_y.iter() {
        let mut sorted_x = vec.clone();
        sorted_x.sort();

        let mut chunk = PlatformChunk::default();
        chunk.start = sorted_x[0];
        chunk.end = sorted_x[sorted_x.len() - 1];

        commands.entity(common_parent).with_children(|parent| {
            parent.spawn((
                OneWayTile,
                Collider::cuboid((chunk.end - chunk.start + 1) as f32 * 4., 4.),
                RigidBody::Fixed,
                TransformBundle::from_transform(Transform::from_xyz(
                    ((chunk.start + chunk.end) as f32 / 2.0) * 8.0,
                    *y_level as f32 * 8.0,
                    0.0
                )),

                ONE_WAY_PLATFORMS_COLLISION_GROUP
            ));
        });
    }
}

pub fn enable_one_way_colliders(
    mut query: Query<(Entity, &mut KinematicCharacterController), (With<Player>, Without<s::Crouch>)>,
    mut colliders: Query<(&Collider, &GlobalTransform), With<OneWayTile>>,
    rapier: Res<RapierContext>
) {
    for (player, mut cc) in query.iter_mut() {
        for (collider, collider_pos) in colliders.iter_mut() {
            let pos = collider_pos.translation()
                / collider_pos.to_scale_rotation_translation().0;


            // UPCAST
            let ix = rapier.cast_shape(
                Vect::new(pos.x, pos.y),
                Rot::default(),
                Vect::new(0.0, 1.0),
                collider,
                PLAYER_TERMINAL_VELOCITY.abs() + PLAYER_SIZE_PX.y / 2.0,
                QueryFilter {
                    predicate: Some(&|x: Entity| x == player),
                    ..default()
                }
            );

            // If the player is above the platform:
            if ix.is_some() && ix.unwrap().1.toi > 1.0 {
                cc.filter_groups = Some(ALL_PLATFORMS_INTERACTION_GROUP);
            }

            // DOWNCAST
            let below = rapier.cast_shape(
                Vect::new(pos.x, pos.y),
                Rot::default(),
                Vect::new(0.0, -1.0),
                collider,
                PLAYER_TERMINAL_VELOCITY.abs() + PLAYER_SIZE_PX.y / 2.0,
                QueryFilter {
                    predicate: Some(&|x: Entity| x == player),
                    ..default()
                }
            );

            // If the player is below the platform:
            if below.is_some() {
                cc.filter_groups = Some(SOLIDS_INTERACTION_GROUP);
            }
        }
    }
}

pub fn disable_one_way_colliders(
    mut player_query: Query<&mut KinematicCharacterController, Added<s::Crouch>>,
) {
    for mut cc in player_query.iter_mut() {
        cc.filter_groups = Some(SOLIDS_INTERACTION_GROUP);
    }
}