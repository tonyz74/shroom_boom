use bevy::prelude::*;
use std::collections::HashMap;
use bevy_ecs_ldtk::prelude::*;
use crate::assets::BossAssets;
use crate::bossfight::{BossBundle, BossConfig};
use crate::level::{coord, LevelInfo, util};
use crate::level::consts::RENDERED_TILE_SIZE;
use crate::state::GameState;


#[derive(Component, Copy, Clone, Default)]
pub struct BoomRegionMarker;

#[derive(Bundle, Default, LdtkEntity)]
pub struct BoomRegionBundle {
    marker: BoomRegionMarker,

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
        .register_ldtk_entity::<BoomRegionBundle>("BoomRegion")
        .register_ldtk_entity::<BossSpawnpointBundle>("BossSpawnpoint")
        .add_system_set(
            SystemSet::on_update(GameState::LevelTransition)
                .with_system(spawn_boss)
        );
}

fn spawn_boss(
    mut commands: Commands,
    boom_regions: Query<&EntityInstance, Added<BoomRegionMarker>>,
    boss: Query<&EntityInstance, Added<BossSpawnpointMarker>>,
    assets: Res<BossAssets>,
    lvl_info: Res<LevelInfo>,
) {
    let mut boom_region_map = HashMap::new();
    for inst in boom_regions.iter() {
        boom_region_map.insert(
            inst.iid.clone(),
            coord::grid_coords_to_region(inst, lvl_info.grid_size)
        );
    }

    for inst in boss.iter() {
        let mut boss = BossBundle::from_assets(&assets);

        boss.config = {
            let (summon_base, charge_left, charge_right, hover_base) = {
                let mut v: [Vec2; 4] = [Vec2::ZERO; 4];

                for i in 0..=3 {
                    let p = util::val_expect_point(&inst.field_instances[i].value).unwrap();
                    println!("{:?} {:?}", i, p);
                    v[i] = coord::grid_coord_to_translation(p, lvl_info.grid_size.as_ivec2());
                }

                (v[0], v[1], v[2], v[3])
            };

            let boom_region = {
                let id = util::val_expect_ent_ref(&inst.field_instances[4].value).unwrap();
                boom_region_map[&id.entity_iid]
            };

            let rightmost = charge_right - Vec2::new(256.0 - RENDERED_TILE_SIZE, 0.0);
            let leftmost = charge_left + Vec2::new(256.0, 0.0);

            BossConfig {
                boom_region,
                summon_base,
                hover_base,

                charge_left: leftmost,
                charge_right: rightmost,

                relocate_point: rightmost,

                x_min: charge_left.x,
                x_max: charge_right.x
            }
        };

        println!(
            "config:\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}",
            boss.config.relocate_point,
            boss.config.charge_left,
            boss.config.charge_right,
            boss.config.hover_base,
            boss.config.summon_base,
            boss.config.boom_region
        );

        boss.sprite_sheet.transform.translation = coord::grid_coord_to_translation(
            inst.grid,
            lvl_info.grid_size.as_ivec2()
        ).extend(0.0);

        BossBundle::spawn(&mut commands, boss);
    }
}