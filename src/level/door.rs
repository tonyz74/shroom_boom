use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::enemies::Enemy;
use crate::state::GameState;

#[derive(Default, Component)]
pub struct DoorTileSpawnMarker;

#[derive(Default, Bundle, LdtkIntCell)]
pub struct DoorTileBundle {
    marker: DoorTileSpawnMarker
}

#[derive(Component, Default, Debug, Copy, Clone)]
pub struct DoorTile {
    pub cleared: bool
}

pub fn register_doors(app: &mut App) {
    app
        .register_ldtk_int_cell_for_layer::<DoorTileBundle>("SpecialTiles", 1)

        .add_system_set(
            SystemSet::on_update(GameState::LevelTransition)
                .with_system(spawn_doors)
        )
        .add_system_set(
            SystemSet::on_update(GameState::Gameplay)
                .with_system(clear_doors_on_enemy_deaths)
        );
}

fn spawn_doors(
    mut commands: Commands,
    q: Query<Entity, Added<DoorTileSpawnMarker>>
) {
    for e in q.iter() {
        commands.entity(e).with_children(|parent| {
            parent.spawn((
                DoorTile { cleared: false },
                Collider::cuboid(4., 4.),
                RigidBody::Fixed,
                TransformBundle::default()
            ));
        });
    }
}

fn clear_doors_on_enemy_deaths(
    mut commands: Commands,
    enemies: Query<&Enemy>,
    mut doors: Query<(Entity, &mut DoorTile)>
) {
    if !enemies.is_empty() {
        return;
    }

    for (entity, mut door) in doors.iter_mut() {
        door.cleared = true;

        if let Some(mut cmd) = commands.get_entity(entity) {
            cmd.insert(Sensor);
        }
    }
}