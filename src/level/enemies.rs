use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    state::GameState,
    assets::SnakeEnemyAssets,
    enemies::flower::FlowerEnemyBundle,
    pathfind::PatrolRegion,
    level::{util, coord, consts::TILE_SIZE},
};

use std::collections::HashMap;
use crate::level::LevelInfo;

#[derive(Component, Default)]
pub struct EnemySpawnpointMarker;

#[derive(Component, Default)]
pub struct PatrolRegionMarker;

#[derive(Bundle, LdtkEntity)]
pub struct EnemySpawnpointBundle {
    marker: EnemySpawnpointMarker,
    #[from_entity_instance]
    instance: EntityInstance
}

#[derive(Bundle, LdtkEntity)]
pub struct PatrolRegionBundle {
    marker: PatrolRegionMarker,
    #[from_entity_instance]
    instance: EntityInstance
}

pub fn register_enemy_spawnpoints(app: &mut App) {
    app
        .register_ldtk_entity::<PatrolRegionBundle>("PatrolRegion")
        .register_ldtk_entity::<EnemySpawnpointBundle>("EnemySpawnpoint")
        .add_system_set(
            SystemSet::on_update(GameState::LevelTransition)
                .with_system(spawn_enemies)
        );
}

fn spawn_enemies(
    mut commands: Commands,
    enemies: Query<&EntityInstance, Added<EnemySpawnpointMarker>>,
    patrol_regions: Query<&EntityInstance, Added<PatrolRegionMarker>>,
    snake_assets: Res<SnakeEnemyAssets>,
    lvl_info: Res<LevelInfo>
) {
    let mut patrol_regions_map = HashMap::new();

    for inst in patrol_regions.iter() {
        let reg_dim = IVec2::new(
            (inst.width as f32 / TILE_SIZE) as i32,
            (inst.height as f32 / TILE_SIZE) as i32
        );

        let tl = GridCoords::new(inst.grid.x, inst.grid.y);
        let br = GridCoords::new(inst.grid.x + reg_dim.x, inst.grid.y + reg_dim.y);


        let region = PatrolRegion {
            tl: coord::grid_coord_to_translation(tl.into(), lvl_info.grid_size.as_ivec2()),
            br: coord::grid_coord_to_translation(br.into(), lvl_info.grid_size.as_ivec2()),
        };

        patrol_regions_map.insert(inst.iid.clone(), region);
    }

    for inst in enemies.iter() {
        let e_ref = util::val_expect_ent_ref(&inst.field_instances[3].value).unwrap();
        let patrol_region = patrol_regions_map[&e_ref.entity_iid];

        let mut enemy = FlowerEnemyBundle::from_assets(&snake_assets);

        {
            let path = &mut enemy.enemy.path;
            path.region = patrol_region;
        }

        {
            let translation = &mut enemy.enemy.sprite_sheet.transform.translation;
            *translation = coord::grid_coord_to_translation(
                inst.grid, lvl_info.grid_size.as_ivec2()
            ).extend(1.0);
        }

        commands.spawn(enemy);
    }
}