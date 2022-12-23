use bevy::prelude::*;
use bevy_ecs_ldtk::GridCoords;
use crate::{
    state::GameState,
    enemies::Enemy,
    pathfind::{Pathfinder, PathfindingGrid}
};
use crate::enemies::flower;

#[derive(Component, Default)]
pub struct CrawlPathfinder {
    pub needs_jump: bool
}

pub fn register_crawl_pathfinders(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(crawl_pathfinder_move)
    );
}

use bevy_rapier2d::prelude::*;
use bevy::math::Vec3Swizzles;

fn crawl_pathfinder_move(
    transforms: Query<&GlobalTransform>,
    mut pathfinders: Query<(
        Entity,
        &Collider,
        &mut Enemy,
        &Pathfinder,
        &mut CrawlPathfinder
    )>,
    rapier: Res<RapierContext>
) {
    for (ent, collider, mut enemy, pathfinder, mut crawl) in pathfinders.iter_mut() {
        let self_transform = transforms.get(ent).unwrap();
        let self_pos = self_transform.translation();

        if let Some(target) = pathfinder.target {
            let target_transform = transforms.get(target).unwrap();
            let target_pos = target_transform.translation();

            if (target_pos.x - self_pos.x).abs() <= 2.0 {
                enemy.vel.x = 0.0;
                return;
            }

            let dir = Vec2::new((target_pos - self_pos).x, 0.0).normalize();
            enemy.vel.x = dir.x * 2.0;


            let ix = rapier.cast_shape(
                (self_pos.xy() + Vec2::new(0.0, 0.0)).into(),
                Rot::default(),
                dir.into(),
                collider,
                6.0 + 1.0,
                QueryFilter {
                    flags: QueryFilterFlags::ONLY_FIXED | QueryFilterFlags::EXCLUDE_SENSORS,
                    ..default()
                }
            );


            if ix.is_some() {
                enemy.vel.x = 0.0;
                crawl.needs_jump = true;
            } else {
                crawl.needs_jump = false;
            }
        } else {
            enemy.vel.x = 0.0;
        }
    }
}