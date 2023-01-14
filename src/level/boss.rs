use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use crate::assets::BossAssets;
use crate::bossfight::BossBundle;
use crate::level::{coord, LevelInfo};
use crate::state::GameState;

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
        .register_ldtk_entity::<BossSpawnpointBundle>("BossSpawnpoint")
        .add_system_set(
            SystemSet::on_update(GameState::LevelTransition)
                .with_system(spawn_boss)
        );
}

fn spawn_boss(
    mut commands: Commands,
    boss: Query<&EntityInstance, Added<BossSpawnpointMarker>>,
    assets: Res<BossAssets>,
    lvl_info: Res<LevelInfo>,
) {
    for inst in boss.iter() {
        let mut boss = BossBundle::from_assets(&assets);
        boss.sprite_sheet.transform.translation = coord::grid_coord_to_translation(
            inst.grid,
            lvl_info.grid_size.as_ivec2()
        ).extend(0.0);

        BossBundle::spawn(&mut commands, boss);
    }
}