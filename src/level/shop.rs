use bevy::prelude::*;
use bevy_ecs_ldtk::EntityInstance;
use bevy_ecs_ldtk::prelude::*;
use crate::assets::ShopAssets;
use crate::common::AnimTimer;
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
    q: Query<&EntityInstance, Added<ShopSpawnpointMarker>>,
    lvl_info: Res<LevelInfo>
) {
    for inst in q.iter() {
        println!("got instance");

        let pos_vec3 = coord::grid_coord_to_translation(
            inst.grid,
            lvl_info.grid_size.as_ivec2()
        ).extend(0.0);

        commands.spawn(ShopBundle {
            sprite_sheet: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::new(180.0, 148.0)),
                    ..default()
                },
                texture_atlas: assets.shopkeeper.tex.clone(),
                transform: Transform::from_translation(pos_vec3),
                ..default()
            },
            anim_timer: AnimTimer::from_seconds(assets.shopkeeper.speed),
            interact: Interact {
                max_dist: 128.0,
                ..default()
            },
            ..default()
        });
    }
}
