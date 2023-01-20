pub mod coord;
pub mod consts;

pub mod solid;
pub mod one_way;
pub mod exit;
pub mod transition;
pub mod enemies;
pub mod util;
pub mod door;
pub mod boss;
pub mod shop;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use crate::level::consts::TILE_SIZE;
use crate::state::GameState;

#[derive(Resource, Default, Copy, Clone)]
pub struct LevelInfo {
    pub cell_size: Vec2,
    pub grid_size: Vec2,
}

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

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(LdtkPlugin)
            .insert_resource(LdtkSettings {
                int_grid_rendering: IntGridRendering::Invisible,
                level_background: LevelBackground::Nonexistent,
                ..default()
            })
            .init_resource::<LevelInfo>()
            .insert_resource(LevelSelection::Index(0))
            .register_ldtk_entity::<PlayerTileBundle>("EntryPoint");

        solid::register_solid_tile(app);
        one_way::register_one_way_tile(app);
        exit::register_exit_entity(app);
        transition::register_transition_systems(app);
        enemies::register_enemy_spawnpoints(app);
        door::register_doors(app);
        boss::register_boss_spawnpoints(app);
        shop::register_shop_spawnpoints(app);

        app.add_system_set(
            SystemSet::on_enter(GameState::LevelTransition)
                .with_system(load_level)
        );

        app.add_system_set(
            SystemSet::on_update(GameState::LevelTransition)
                .with_system(move_player)
                .with_system(refresh_level)
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

pub fn refresh_level(
    levels: Query<&Handle<LdtkAsset>>,
    assets: Res<Assets<LdtkAsset>>,
    sel: Res<LevelSelection>,

    mut lvl_info: ResMut<LevelInfo>
) {
    if levels.is_empty() || assets.is_empty() {
        return;
    }

    let lvl = assets.get(levels.single()).unwrap().get_level(&sel).unwrap();

    lvl_info.cell_size = Vec2::new(TILE_SIZE, TILE_SIZE);
    lvl_info.grid_size = IVec2::new(lvl.px_wid, lvl.px_hei).as_vec2() / lvl_info.cell_size;
}

pub fn reconfigure_region_to_fit_level(
    mut region: Query<(&mut Transform, &mut Collider), With<LevelRegion>>,
    lvl_info: Res<LevelInfo>
) {
    for (mut region, mut collider) in region.iter_mut() {
        let half_extents = (lvl_info.grid_size * lvl_info.cell_size) / 2.0;
        region.translation = Vec3::new(half_extents.x, half_extents.y, 0.0);
        *collider = Collider::cuboid(half_extents.x + 10.0, half_extents.y + 10.0);
    }
}

use crate::player::Player;
use crate::player::consts::PLAYER_SIZE_PX;

#[derive(Component)]
pub struct FinishedTransitioning;

fn move_player(
    mut commands: Commands,
    transition: Res<transition::LevelTransition>,
    mut q: Query<(Entity, &mut Transform), With<Player>>,
    pos: Query<&EntityInstance, Added<PlayerTileMarker>>,
    lvl_info: Res<LevelInfo>,
) {
    for inst in pos.iter() {
        let entry_point_id = util::val_expect_i32(&inst.field_instances[0].value).unwrap();

        if entry_point_id as u32 != transition.entry_point_id {
            continue;
        }

        let (e, mut tf) = q.single_mut();
        tf.translation = coord::grid_coord_to_translation(
            inst.grid,
            lvl_info.grid_size.as_ivec2()
        ).extend(1.0);

        tf.translation.x += PLAYER_SIZE_PX.x / 2.0;

        commands.entity(e).insert(FinishedTransitioning);
    }
}