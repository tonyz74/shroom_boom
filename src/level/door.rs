use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::anim::{AnimationChangeEvent, AnimationPlugin, Animator};
use crate::anim::map::AnimationMap;
use crate::assets::LevelAssets;
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
    q: Query<Entity, Added<DoorTileSpawnMarker>>,
    assets: Res<LevelAssets>
) {
    for e in q.iter() {
        commands.entity(e).with_children(|parent| {
            parent.spawn((
                DoorTile { cleared: false },
                Collider::cuboid(4., 4.),
                Animator::new(assets.anims["SOLID"].clone()),
                assets.anims.clone(),

                SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        custom_size: Some(Vec2::new(8.0, 8.0)),
                        ..default()
                    },
                    texture_atlas: assets.anims["SOLID"].tex.clone(),
                    transform: Transform::from_xyz(0.0, 0.0, 5.0),
                    ..default()
                }
            ));
        });
    }
}

fn clear_doors_on_enemy_deaths(
    mut commands: Commands,
    enemies: Query<&Enemy>,
    mut doors: Query<(Entity, &mut DoorTile, &Animator)>,
    mut ev: EventWriter<AnimationChangeEvent>,
    assets: Res<LevelAssets>
) {
    if !enemies.is_empty() {
        return;
    }

    for (entity, mut door, animator) in doors.iter_mut() {
        door.cleared = true;

        ev.send(AnimationChangeEvent {
            e: entity,
            new_anim: assets.anims["DISINTEGRATE"].clone()
        });

        if animator.anim.name == "DISINTEGRATE" && animator.total_looped == 1 {
            commands.entity(entity).despawn_recursive();
        }
    }
}