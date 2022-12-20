use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::state::GameState;

#[derive(Default, Component)]
pub struct SolidTileSpawnMarker;

#[derive(Bundle, LdtkIntCell)]
pub struct SolidTileBundle {
    marker: SolidTileSpawnMarker
}

#[derive(Component)]
pub struct SolidTile;

pub fn register_solid_tile(app: &mut App) {
    app
        .register_ldtk_int_cell_for_layer::<SolidTileBundle>("Tiles", 1)
        .add_system_set(
            SystemSet::on_update(GameState::LevelTransition)
                .with_system(add_solid_tiles)
        );
}

pub fn add_solid_tiles(
    mut commands: Commands,
    colliders: Query<Entity, Added<SolidTileSpawnMarker>>
) {
    for e in colliders.iter() {
        commands.entity(e).with_children(|parent| {
            parent.spawn((
                SolidTile,
                Collider::cuboid(4., 4.),
                RigidBody::Fixed,
                TransformBundle::default()
            ));
        });
    }
}