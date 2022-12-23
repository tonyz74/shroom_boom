pub mod coord;
pub mod consts;

mod solid;
mod one_way;
mod exit;
mod transition;
mod enemies;
mod util;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use crate::level::consts::TILE_SIZE;
use crate::state::GameState;

#[derive(Component, Default)]
pub struct PlayerTileMarker;

#[derive(Bundle, LdtkEntity)]
pub struct PlayerTileBundle {
    marker: PlayerTileMarker,
    #[from_entity_instance]
    entity_instance: EntityInstance
}

#[derive(Component)]
pub struct LevelRegion;

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
            .register_ldtk_entity::<PlayerTileBundle>("EntryPoint");

        solid::register_solid_tile(app);
        one_way::register_one_way_tile(app);
        exit::register_exit_entity(app);
        transition::register_transition_systems(app);
        enemies::register_enemy_spawnpoints(app);

        app.add_system_set(
            SystemSet::on_enter(GameState::LevelTransition)
                .with_system(load_level)
        );

        app.add_system_set(
            SystemSet::on_update(GameState::LevelTransition)
                .with_system(move_player)
                .with_system(reconfigure_region_to_fit_level)
        );
    }
}

fn load_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    levels: Query<Entity, With<LevelSet>>
) {
    if !levels.is_empty() {
        return;
    }

    commands.spawn(
        LdtkWorldBundle {
            ldtk_handle: asset_server.load("levels/levels.ldtk"),
            transform: Transform::from_scale(
                Vec3::new(consts::SCALE_FACTOR, consts::SCALE_FACTOR, 1.0)
            ),
            ..default()
        }
    ).with_children(|parent| {
        parent.spawn((
            Sensor,
            RigidBody::Fixed,
            Collider::cuboid(1.0, 1.0),
            TransformBundle::default(),
            LevelRegion
        ));
    });
}

pub fn reconfigure_region_to_fit_level(
    levels: Query<&Handle<LdtkAsset>>,
    assets: Res<Assets<LdtkAsset>>,
    sel: Res<LevelSelection>,

    mut region: Query<(&mut Transform, &mut Collider), With<LevelRegion>>,
) {
    if levels.is_empty() || assets.is_empty() {
        return;
    }

    for (mut region, mut collider) in region.iter_mut() {
        let level = assets.get(levels.single()).unwrap();

        if let Some(lvl) = level.get_level(&sel) {
            let half_extents = Vec2::new(
                lvl.px_wid as f32 / 2.0,
                lvl.px_hei as f32 / 2.0
            );

            region.translation = Vec3::new(half_extents.x, half_extents.y, 0.0);
            *collider = Collider::cuboid(half_extents.x + 10.0, half_extents.y + 10.0);
        }
    }
}

use crate::player::Player;
use crate::player::consts::PLAYER_SIZE_PX;

#[derive(Component)]
pub struct Active;

fn move_player(
    mut commands: Commands,
    level_info: Res<transition::LevelTransition>,
    mut q: Query<(Entity, &mut Transform), With<Player>>,
    pos: Query<&EntityInstance, Added<PlayerTileMarker>>,


    level_sel: Res<LevelSelection>,
    level: Query<&Handle<LdtkAsset>>,
    assets: Res<Assets<LdtkAsset>>,
) {
    if level.is_empty() || assets.is_empty() {
        return;
    }

    let lvl = assets.get(level.single()).unwrap().get_level(&level_sel).unwrap();

    for inst in pos.iter() {
        let entry_point_id = match inst.field_instances[0].value {
            FieldValue::Int(Some(id)) => id,
            _ => panic!()
        };

        if entry_point_id as u32 != level_info.entry_point_id {
            continue;
        }

        let (e, mut tf) = q.single_mut();
        tf.translation = coord::grid_coord_to_translation(
            inst.grid,
            IVec2::new(
                (lvl.px_wid as f32 / TILE_SIZE) as i32,
                (lvl.px_hei as f32 / TILE_SIZE) as i32
            ),
        ).extend(1.0);

        tf.translation.x += PLAYER_SIZE_PX.x / 2.0;

        commands.entity(e).insert(Active);
    }
}