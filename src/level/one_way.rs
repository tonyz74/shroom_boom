use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

use std::collections::HashMap;
use crate::{
    state::GameState,
    input::InputAction,
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
            SystemSet::on_update(GameState::Gameplay)
                .with_system(add_one_way_tiles)
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
            ));
        });
    }
}

pub fn enable_one_way_colliders(
    mut commands: Commands,
    query: Query<Entity, (With<Player>, Without<s::Crouch>)>,
    mut colliders: Query<(Entity, &Collider, &GlobalTransform), With<OneWayTile>>,
    rapier: Res<RapierContext>
) {
    for player in query.iter() {
        for (entity, collider, collider_pos) in colliders.iter_mut() {
            let pos = collider_pos.translation()
                / collider_pos.to_scale_rotation_translation().0;

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
                commands.entity(entity).remove::<Sensor>();
            }

        }
    }
}

pub fn disable_one_way_colliders(
    mut commands: Commands,
    player_query: Query<&Player, Added<s::Crouch>>,
    platforms: Query<Entity, With<OneWayTile>>
) {
    for player in player_query.iter() {
        let _ = player;

        for platform in platforms.iter() {
            commands.entity(platform).insert(Sensor);
        }
    }
}