use bevy::prelude::*;
use bevy_ecs_ldtk::EntityInstance;
use bevy_ecs_ldtk::prelude::*;
use crate::assets::{ShopAssets, UiAssets};
use crate::interact::Interact;
use crate::level::{coord, LevelInfo};
use crate::shop::ShopBundle;
use crate::state::GameState;

#[derive(Component, Copy, Clone, Default)]
pub struct ShopSpawnpointMarker;

#[derive(Bundle, Default, LdtkEntity)]
pub struct ShopSpawnpointBundle {
    marker: ShopSpawnpointMarker,
    #[from_entity_instance]
    inst: EntityInstance
}

pub fn register_shop_spawnpoints(app: &mut App) {
    app
        .register_ldtk_entity::<ShopSpawnpointBundle>("ShopSpawnpoint")
        .add_system_set(
            SystemSet::on_update(GameState::LevelTransition)
                .with_system(spawn_shops)
        );
}

fn spawn_shops(
    mut commands: Commands,
    assets: Res<ShopAssets>,
    ui_assets: Res<UiAssets>,
    q: Query<&EntityInstance, Added<ShopSpawnpointMarker>>,
    lvl_info: Res<LevelInfo>
) {
    for inst in q.iter() {
        let pos_vec2 = coord::grid_coord_to_translation(
            inst.grid,
            lvl_info.grid_size.as_ivec2()
        );
        info!("Spawning shop at {:?}", pos_vec2);

        commands.spawn(ShopBundle::new(&assets, &ui_assets, pos_vec2));
    }
}
