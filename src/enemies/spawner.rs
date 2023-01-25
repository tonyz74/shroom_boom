use std::ops::Range;
use bevy::prelude::*;
use crate::assets::{DandelionEnemyAssets, FlowerEnemyAssets, PumpkinEnemyAssets, TumbleweedEnemyAssets};
use crate::enemies::dandelion::DandelionEnemyBundle;
use crate::enemies::dandelion::stats::{DANDELION_EASY, DANDELION_HARD, DANDELION_MEDIUM};
use crate::enemies::EnemyBundle;
use crate::enemies::flower::FlowerEnemyBundle;
use crate::enemies::flower::stats::{FLOWER_EASY, FLOWER_HARD, FLOWER_MEDIUM};
use crate::enemies::pumpkin::PumpkinEnemyBundle;
use crate::enemies::pumpkin::stats::{PUMPKIN_EASY, PUMPKIN_HARD, PUMPKIN_MEDIUM};
use crate::enemies::tumbleweed::stats::{TUMBLEWEED_EASY, TUMBLEWEED_HARD, TUMBLEWEED_MEDIUM};
use crate::enemies::tumbleweed::TumbleweedEnemyBundle;
use crate::pathfind::Region;

#[derive(Copy, Clone, Debug, Component)]
pub enum EnemyDifficulty {
    Easy,
    Medium,
    Hard
}

#[derive(Copy, Clone, Debug, Component)]
pub enum EnemyType {
    Flower,
    Dandelion,
    Pumpkin,
    Tumbleweed,
    Mushroom
}

#[derive(Clone, Component)]
pub struct EnemySpawnEvent {
    pub ty: EnemyType,
    pub coins: i32,
    pub difficulty: EnemyDifficulty,
    pub location: EnemyLocation,
    pub rand_range: Range<f32>,
    pub extra_components: Option<fn(&mut Commands, Entity)>
}

#[derive(Clone, Debug, Component)]
pub struct EnemyLocation {
    pub pos: Vec2,
    pub patrol_region: Region,
}

pub fn register_enemy_spawner(app: &mut App) {
    app
        .add_event::<EnemySpawnEvent>()
        .add_system(spawn_enemies);
}




fn configure_enemy(enemy: &mut EnemyBundle, ev: &EnemySpawnEvent) {
    enemy.sprite_sheet.transform.translation.x = ev.location.pos.x;
    enemy.sprite_sheet.transform.translation.y = ev.location.pos.y;
    enemy.path.pathfinder.region = ev.location.patrol_region;
    enemy.coins.total_value = ev.coins;
}

fn spawn_enemies(
    mut commands: Commands,
    mut events: EventReader<EnemySpawnEvent>,

    flower_assets: Res<FlowerEnemyAssets>,
    pumpkin_assets: Res<PumpkinEnemyAssets>,
    dandelion_assets: Res<DandelionEnemyAssets>,
    tumbleweed_assets: Res<TumbleweedEnemyAssets>
) {
    for enemy in events.iter() {
        let id = match enemy.ty {
            EnemyType::Flower => {
                let mut bundle = FlowerEnemyBundle::from_assets(&flower_assets);
                configure_enemy(&mut bundle.enemy, &enemy);

                let stats = match enemy.difficulty {
                    EnemyDifficulty::Easy => FLOWER_EASY,
                    EnemyDifficulty::Medium => FLOWER_MEDIUM,
                    EnemyDifficulty::Hard => FLOWER_HARD
                }.randomized(enemy.rand_range.clone());

                FlowerEnemyBundle::spawn_with_stats(&mut commands, bundle, stats)
            },

            EnemyType::Dandelion => {
                let mut bundle = DandelionEnemyBundle::from_assets(&dandelion_assets);
                configure_enemy(&mut bundle.enemy, &enemy);

                let stats = match enemy.difficulty {
                    EnemyDifficulty::Easy => DANDELION_EASY,
                    EnemyDifficulty::Medium => DANDELION_MEDIUM,
                    EnemyDifficulty::Hard => DANDELION_HARD
                }.randomized(enemy.rand_range.clone());

            DandelionEnemyBundle::spawn_with_stats(&mut commands, bundle, stats)
            },

            EnemyType::Pumpkin => {
                let mut bundle = PumpkinEnemyBundle::from_assets(&pumpkin_assets);
                configure_enemy(&mut bundle.enemy, &enemy);

                let stats = match enemy.difficulty {
                    EnemyDifficulty::Easy => PUMPKIN_EASY,
                    EnemyDifficulty::Medium => PUMPKIN_MEDIUM,
                    EnemyDifficulty::Hard => PUMPKIN_HARD
                }.randomized(enemy.rand_range.clone());

                PumpkinEnemyBundle::spawn_with_stats(&mut commands, bundle, stats)
            },


            EnemyType::Tumbleweed => {
                let mut bundle = TumbleweedEnemyBundle::from_assets(&tumbleweed_assets);
                configure_enemy(&mut bundle.enemy, &enemy);

                let stats = match enemy.difficulty {
                    EnemyDifficulty::Easy => TUMBLEWEED_EASY,
                    EnemyDifficulty::Medium => TUMBLEWEED_MEDIUM,
                    EnemyDifficulty::Hard => TUMBLEWEED_HARD
                }.randomized(enemy.rand_range.clone());

                TumbleweedEnemyBundle::spawn_with_stats(&mut commands, bundle, stats)
            },

            _ => {
                panic!("Unknown enemy type {:?}", enemy.ty);
            }
        };

        if let Some(func) = enemy.extra_components {
            func(&mut commands, id);
        }
    }
}