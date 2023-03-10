use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    state::GameState,
    enemies::Enemy,
    entity_states::*,
    pathfind::{
        Pathfinder,
        walk::WalkPathfinder,
    }
};

use std::collections::HashSet;
use crate::pathfind::{
    Patrol,
    walk_pathfinder_get_suitable_target,
    walk_pathfinder_jump_if_needed,
    walk_pathfinder_stop_if_colliding_enemy_stopped
};
use crate::util::{Facing, FacingX};

#[derive(Component, Default, Debug, Copy, Clone)]
pub struct MeleePathfinder;

pub fn register_melee_pathfinders(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(melee_pathfinder_move)
    );
}

fn melee_pathfinder_move(
    transforms: Query<&GlobalTransform, With<Enemy>>,
    mut pathfinders: Query<(
        Entity,
        &Collider,
        &mut Enemy,
        &mut Pathfinder,
        &mut WalkPathfinder,
        &mut Facing,
        &Patrol
    ), (Without<Hurt>, Without<Die>, With<MeleePathfinder>)>,
    rapier: Res<RapierContext>,
) {
    let mut colliding_enemies: HashSet<(Entity, Entity)> = HashSet::new();

    for (ent, collider, mut enemy, mut pathfinder, mut walk, mut facing, patrol) in pathfinders.iter_mut() {
        if !pathfinder.active {
            continue;
        }

        let self_transform = transforms.get(ent).unwrap();

        let self_pos = Vec2::new(
            self_transform.translation().x,
            self_transform.translation().y,
        );

        if let Some(mut target_pos) = pathfinder.target {
            target_pos = walk_pathfinder_get_suitable_target(self_pos, target_pos, &pathfinder);

            if (target_pos.x - self_pos.x).abs() <= 2.0 {
                if patrol.lost_target {
                    pathfinder.target = None;
                }

                enemy.vel.x = 0.0;
                continue;
            }

            let dir = Vec2::new((target_pos - self_pos).x, 0.0).normalize();
            enemy.vel.x = dir.x * pathfinder.speed;

            if dir.x < 0.0 {
                facing.x = FacingX::Left;
            } else if dir.x > 0.0 {
                facing.x = FacingX::Right;
            }

            walk_pathfinder_jump_if_needed(
                Vec2::new(self_pos.x, self_pos.y),
                dir.into(),
                collider,
                &mut enemy,
                &pathfinder,
                &mut walk,
                &rapier
            );
        }

        if pathfinder.target.is_some() {
            rapier.intersections_with_shape(
                self_pos,
                Rot::default(),
                collider,
                QueryFilter {
                    flags: QueryFilterFlags::ONLY_KINEMATIC,
                    exclude_rigid_body: Some(ent),
                    ..default()
                },
                |collision| {
                    if transforms.contains(collision) {
                        colliding_enemies.insert((ent, collision));
                    }

                    true
                }
            );
        }
    }

    for (c1, c2) in colliding_enemies.iter() {
        walk_pathfinder_stop_if_colliding_enemy_stopped(*c1, *c2, &mut pathfinders);
    }
}