use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    level::coord,
    state::GameState,
};
use crate::assets::SnakeEnemyAssets;
use crate::enemies::snake::SnakeEnemyBundle;

#[derive(Component, Default)]
pub struct EnemySpawnpointMarker;

#[derive(Bundle, LdtkEntity)]
pub struct EnemySpawnpointBundle {
    marker: EnemySpawnpointMarker,
    #[from_entity_instance]
    instance: EntityInstance
}

pub fn register_enemy_spawnpoints(app: &mut App) {
    app
        .register_ldtk_entity::<EnemySpawnpointBundle>("EnemySpawnpoint")
        .add_system_set(
            SystemSet::on_update(GameState::LevelTransition)
                .with_system(spawn_enemies)
        );
}


fn spawn_enemies(
    mut commands: Commands,
    q: Query<&EntityInstance, Added<EnemySpawnpointMarker>>,
    snake_assets: Res<SnakeEnemyAssets>
) {
    for inst in q.iter() {
        let mut enemy = SnakeEnemyBundle::from_assets(&snake_assets);

        enemy.enemy.sprite_sheet.transform.translation = coord::grid_coord_to_translation(
            inst.grid,
            IVec2::new(48, 32)
        ).extend(1.0);

        enemy.enemy.sprite_sheet.transform.translation.x += 32.0;

        commands.spawn(enemy);
    }
}