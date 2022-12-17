mod consts;
mod solid;
mod one_way;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    state::GameState
};

#[derive(Component, Default)]
pub struct CollisionTile;

#[derive(Component, Default)]
pub struct IntGridMarker;

#[derive(Bundle, LdtkIntCell)]
pub struct CollisionTileBundle {
    marker: IntGridMarker
}

#[derive(Component, Default)]
pub struct PlayerTileMarker;

#[derive(Bundle, LdtkEntity)]
pub struct PlayerTileBundle {
    marker: PlayerTileMarker,
    #[from_entity_instance]
    entity_instance: EntityInstance
}

pub struct LevelLoaderPlugin;

impl Plugin for LevelLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(LdtkPlugin)
            .insert_resource(LdtkSettings {
                level_background: LevelBackground::Nonexistent,
                ..default()
            })
            .insert_resource(LevelSelection::Index(0))
            .register_ldtk_entity::<PlayerTileBundle>("Player");

        solid::register_solid_tile(app);
        one_way::register_one_way_tile(app);

        app.add_system_set(
            SystemSet::on_enter(GameState::Gameplay)
                .with_system(load_level)
        );

        app.add_system_set(
            SystemSet::on_update(GameState::Gameplay)
                .with_system(move_player)
        );
    }
}

fn load_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn(
        LdtkWorldBundle {
            ldtk_handle: asset_server.load("levels/levels.ldtk"),
            transform: Transform::from_scale(
                Vec3::new(
                    consts::RENDERED_TILE_SIZE / consts::TILE_SIZE,
                    consts::RENDERED_TILE_SIZE / consts::TILE_SIZE,
                    1.0
                )
            ),
            ..default()
        }
    );
}

use crate::player::Player;
use crate::player::consts::PLAYER_SIZE_PX;

fn move_player(
    mut q: Query<&mut Transform, With<Player>>,
    pos: Query<&EntityInstance, Added<PlayerTileMarker>>
) {
    for p in pos.iter() {
        info!("{:?}", p);

        let mut tf = q.single_mut();
        tf.translation = (Vec2::new(p.grid.x as f32, 32.0 - p.grid.y as f32) * 32.0).extend(1.0);
        tf.translation.x += PLAYER_SIZE_PX.x / 2.0;
    }
}