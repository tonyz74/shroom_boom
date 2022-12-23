use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use bevy_rapier2d::prelude::*;
use std::collections::HashMap;

use crate::{
    level::consts::ONE_WAY_PLATFORMS_COLLISION_GROUP,
    state::GameState,
    player::{
        Player,
        state_machine as s,
    }
};
use crate::level::consts::SOLIDS_COLLISION_GROUP;

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
    query: Query<Entity, (With<Player>, Without<s::Crouch>)>,
    mut colliders: Query<(&mut Collider, &mut CollisionGroups, &GlobalTransform), With<OneWayTile>>,
    rapier: Res<RapierContext>
) {
    for player in query.iter() {
        for (collider, mut collision_groups, collider_pos) in colliders.iter_mut() {
            let pos = collider_pos.translation()
                / collider_pos.to_scale_rotation_translation().0;

            let cast = rapier.cast_shape(
                Vect::new(pos.x, pos.y),
                Rot::default(),
                Vect::new(0.0, 1.0),
                &collider,
                Real::MAX,
                QueryFilter {
                    predicate: Some(&|x: Entity| x == player),
                    ..default()
                }
            );

            if let Some((_, Toi { toi, .. })) = cast {
                if toi < 0.2 {
                    *collision_groups = ONE_WAY_PLATFORMS_COLLISION_GROUP;
                } else {
                    *collision_groups = SOLIDS_COLLISION_GROUP;
                }
            }

            let cast = rapier.cast_shape(
                Vect::new(pos.x, pos.y),
                Rot::default(),
                Vect::new(0.0, -1.0),
                &collider,
                Real::MAX,
                QueryFilter {
                    predicate: Some(&|x: Entity| x == player),
                    ..default()
                }
            );

            if cast.is_some() {
                *collision_groups = ONE_WAY_PLATFORMS_COLLISION_GROUP;
            }
        }
    }
}

pub fn disable_one_way_colliders(
    player_query: Query<&Player, Added<s::Crouch>>,
    mut one_way_platforms: Query<&mut CollisionGroups, With<OneWayTile>>
) {
    for _ in player_query.iter() {
        for mut platform in one_way_platforms.iter_mut() {
            *platform = ONE_WAY_PLATFORMS_COLLISION_GROUP;
        }
    }
}