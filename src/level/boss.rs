use bevy::prelude::*;
use std::collections::HashMap;
use bevy_ecs_ldtk::prelude::*;
use crate::assets::BossAssets;
use crate::bossfight::{BossBundle, BossConfig};
use crate::level::{coord, LevelInfo, util};
use crate::level::consts::RENDERED_TILE_SIZE;
use crate::state::GameState;


#[derive(Component, Copy, Clone, Default)]
pub struct RegionMarker;

#[derive(Bundle, Default, LdtkEntity)]
pub struct RegionBundle {
    marker: RegionMarker,

    #[from_entity_instance]
    inst: EntityInstance
}


#[derive(Component, Copy, Clone, Default)]
pub struct BossSpawnpointMarker;

#[derive(Bundle, Default, LdtkEntity)]
pub struct BossSpawnpointBundle {
    marker: BossSpawnpointMarker,

    #[from_entity_instance]
    inst: EntityInstance
}

pub fn register_boss_spawnpoints(app: &mut App) {
    app
        .register_ldtk_entity::<RegionBundle>("BoomRegion")
        .register_ldtk_entity::<RegionBundle>("SummonRegion")
        .register_ldtk_entity::<BossSpawnpointBundle>("BossSpawnpoint")
        .add_system_set(
            SystemSet::on_update(GameState::LevelTransition)
                .with_system(spawn_boss)
        );
}

fn spawn_boss(
    mut commands: Commands,
    regions: Query<&EntityInstance, Added<RegionMarker>>,
    boss: Query<&EntityInstance, Added<BossSpawnpointMarker>>,
    assets: Res<BossAssets>,
    lvl_info: Res<LevelInfo>,
) {
    let mut region_map = HashMap::new();
    for inst in regions.iter() {
        region_map.insert(
            inst.iid.clone(),
            coord::grid_coords_to_region(inst, lvl_info.grid_size)
        );
    }

    for inst in boss.iter() {
        let mut boss = BossBundle::from_assets(&assets);

        boss.config = {
            let (charge_left, charge_right, hover_base, slam_base) = {
                let mut v: [Vec2; 4] = [Vec2::ZERO; 4];

                for i in 0..v.len() {
                    let p = util::val_expect_point(&inst.field_instances[i].value).unwrap();
                    println!("{:?} {:?}", i, p);
                    v[i] = coord::grid_coord_to_translation(p, lvl_info.grid_size.as_ivec2());
                }

                (v[0], v[1], v[2], v[3])
            };

            let boom_region = {
                let id = util::val_expect_ent_ref(&inst.field_instances[4].value).unwrap();
                region_map[&id.entity_iid]
            };

            let summon_region = {
                let id = util::val_expect_ent_ref(&inst.field_instances[5].value).unwrap();
                region_map[&id.entity_iid]
            };

            println!("summon region: {:?}", summon_region);

            let rightmost = charge_right - Vec2::new(256.0 - RENDERED_TILE_SIZE, 0.0);
            let leftmost = charge_left + Vec2::new(256.0, 0.0);
            let bottommost = slam_base + Vec2::new(0.0, 256.0 - RENDERED_TILE_SIZE);

            BossConfig {
                boom_region,
                summon_region,

                hover_base,
                slam_base: bottommost,

                charge_left: leftmost,
                charge_right: rightmost,

                relocate_point: rightmost,

                x_min: charge_left.x,
                x_max: charge_right.x
            }
        };

        boss.sprite_sheet.transform.translation = coord::grid_coord_to_translation(
            inst.grid,
            lvl_info.grid_size.as_ivec2()
        ).extend(0.0);

        BossBundle::spawn(&mut commands, boss);
    }
}