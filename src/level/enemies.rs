use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    state::GameState,
    level::{util, coord},
};

use std::collections::HashMap;
use crate::enemies::spawner::{EnemyDifficulty, EnemyLocation, EnemySpawnEvent, EnemyType};
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
    // mut commands: Commands,
    enemies: Query<&EntityInstance, Added<EnemySpawnpointMarker>>,
    patrol_regions: Query<&EntityInstance, Added<PatrolRegionMarker>>,
    lvl_info: Res<LevelInfo>,

    mut spawns: EventWriter<EnemySpawnEvent>
) {
    let mut patrol_regions_map = HashMap::new();

    for inst in patrol_regions.iter() {
        patrol_regions_map.insert(
            inst.iid.clone(),
            coord::grid_coords_to_region(&inst, lvl_info.grid_size)
        );
    }

    for inst in enemies.iter() {
        let e_ref = util::val_expect_ent_ref(&inst.field_instances[1].value).unwrap();
        let patrol_region = patrol_regions_map[&e_ref.entity_iid];



        let n_coins = util::val_expect_i32(&inst.field_instances[2].value).unwrap();

        let enemy_ty = {
            let enemy_type = match &inst.field_instances[0].value {
                FieldValue::Enum(Some(name)) => name.clone(),
                _ => panic!()
            };

            match enemy_type.as_str() {
                "Flower" => EnemyType::Flower,
                "Pumpkin" => EnemyType::Pumpkin,
                "Dandelion" => EnemyType::Dandelion,
                "Tumbleweed" => EnemyType::Tumbleweed,
                _ => panic!()
            }
        };

        let enemy_difficulty = {
            let d = match &inst.field_instances[3].value {
                FieldValue::Enum(Some(name)) => name.clone(),
                _ => panic!()
            };

            match d.as_str() {
                "Mellow" => EnemyDifficulty::Mellow,
                "Easy" => EnemyDifficulty::Easy,
                "Medium" => EnemyDifficulty::Medium,
                "Hard" => EnemyDifficulty::Hard,
                _ => panic!()
            }
        };
        
        let ev = EnemySpawnEvent {
            ty: enemy_ty,
            coins: n_coins,
            difficulty: enemy_difficulty,
            location: EnemyLocation {
                pos: coord::grid_coord_to_translation(
                    inst.grid, lvl_info.grid_size.as_ivec2()
                ),
                patrol_region
            },
            rand_range: 0.9..1.1,
            extra_components: None
        };

        spawns.send(ev);
    }
}