use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    state::GameState,
    assets::FlowerEnemyAssets,
    enemies::{
        pumpkin::PumpkinEnemyBundle,
        flower::FlowerEnemyBundle
    },
    pathfind::util::Region,
    level::{util, coord, consts::TILE_SIZE},
};

use std::collections::HashMap;
use crate::assets::{DandelionEnemyAssets, PumpkinEnemyAssets};
use crate::coin::drops::CoinDrops;
use crate::enemies::dandelion::DandelionEnemyBundle;
use crate::enemies::EnemyBundle;
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
    lvl_info: Res<LevelInfo>,

    flower_assets: Res<FlowerEnemyAssets>,
    pumpkin_assets: Res<PumpkinEnemyAssets>,
    dandelion_assets: Res<DandelionEnemyAssets>
) {
    let mut patrol_regions_map = HashMap::new();

    for inst in patrol_regions.iter() {
        let reg_dim = IVec2::new(
            (inst.width as f32 / TILE_SIZE) as i32,
            (inst.height as f32 / TILE_SIZE) as i32
        );

        let tl = GridCoords::new(inst.grid.x, inst.grid.y);
        let br = GridCoords::new(inst.grid.x + reg_dim.x, inst.grid.y + reg_dim.y);

        let region = Region {
            tl: coord::grid_coord_to_translation(tl.into(), lvl_info.grid_size.as_ivec2()),
            br: coord::grid_coord_to_translation(br.into(), lvl_info.grid_size.as_ivec2()),
        };

        patrol_regions_map.insert(inst.iid.clone(), region);
    }

    for inst in enemies.iter() {
        let e_ref = util::val_expect_ent_ref(&inst.field_instances[1].value).unwrap();
        let patrol_region = patrol_regions_map[&e_ref.entity_iid];

        let enemy_type = match &inst.field_instances[0].value {
            FieldValue::Enum(Some(name)) => name.clone(),
            _ => panic!()
        };

        let coins = util::val_expect_i32(&inst.field_instances[2].value).unwrap();

        match enemy_type.as_str() {
            "Flower" => {
                spawn_flower(
                    &mut commands,
                    &inst,
                    coins,
                    patrol_region,
                    &lvl_info,
                    &flower_assets
                )
            },
            "Pumpkin" => {
                spawn_pumpkin(
                    &mut commands,
                    &inst,
                    coins,
                    patrol_region,
                    &lvl_info,
                    &pumpkin_assets
                )
            },
            "Dandelion" => {
                spawn_dandelion(
                    &mut commands,
                    &inst,
                    coins,
                    patrol_region,
                    &lvl_info,
                    &dandelion_assets
                )
            },
            _ => panic!()
        }
    }
}


fn configure_enemy(
    enemy: &mut EnemyBundle,
    inst: &EntityInstance,
    coins: i32,
    patrol_region: Region,
    lvl_info: &Res<LevelInfo>,
) {
    enemy.path.pathfinder.region = patrol_region;

    enemy.sprite_sheet.transform.translation = coord::grid_coord_to_translation(
        inst.grid,
        lvl_info.grid_size.as_ivec2()
    ).extend(1.0);

    enemy.coins = CoinDrops { amount: coins };
}

pub fn spawn_flower(
    commands: &mut Commands,
    inst: &EntityInstance,
    coins: i32,
    patrol_region: Region,
    lvl_info: &Res<LevelInfo>,
    assets: &Res<FlowerEnemyAssets>
) {
    let mut enemy = FlowerEnemyBundle::from_assets(&assets);
    configure_enemy(&mut enemy.enemy, inst, coins, patrol_region, lvl_info);

    FlowerEnemyBundle::spawn(commands, enemy);
}

pub fn spawn_pumpkin(
    commands: &mut Commands,
    inst: &EntityInstance,
    coins: i32,
    patrol_region: Region,
    lvl_info: &Res<LevelInfo>,
    assets: &Res<PumpkinEnemyAssets>
) {
    let mut enemy = PumpkinEnemyBundle::from_assets(&assets);
    configure_enemy(&mut enemy.enemy, inst, coins, patrol_region, lvl_info);

    PumpkinEnemyBundle::spawn(commands, enemy);
}

pub fn spawn_dandelion(
    commands: &mut Commands,
    inst: &EntityInstance,
    coins: i32,
    patrol_region: Region,
    lvl_info: &Res<LevelInfo>,
    assets: &Res<DandelionEnemyAssets>
) {
    let mut enemy = DandelionEnemyBundle::from_assets(&assets);
    configure_enemy(&mut enemy.enemy, inst, coins, patrol_region, lvl_info);

    DandelionEnemyBundle::spawn(commands, enemy);
}