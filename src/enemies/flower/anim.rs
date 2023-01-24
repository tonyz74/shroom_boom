use std::time::Duration;
use bevy::prelude::*;
use crate::anim::{AnimationChangeEvent, Animator};
use crate::assets::FlowerEnemyAssets;
use crate::enemies::Enemy;
use crate::enemies::flower::FlowerEnemy;
use crate::enemies::flower::state_machine::Detonate;
use crate::state::GameState;
use crate::entity_states::*;
use crate::pathfind::Patrol;

pub fn register_flower_enemy_animations(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(flower_enemy_idle_on_patrol_pause)
            .with_system(flower_enemy_move_on_patrol_resume)
            .with_system(flower_enemy_move)
            .with_system(flower_enemy_detonate)
    );
}

fn flower_enemy_idle_on_patrol_pause(
    assets: Res<FlowerEnemyAssets>,
    q: Query<(&Enemy, Entity, &Patrol, &Animator), With<FlowerEnemy>>,
    mut ev: EventWriter<AnimationChangeEvent>
) {
    for (enemy, flower, patrol, animator) in q.iter() {

        if patrol.patrol_pause_timer.elapsed() <= Duration::from_secs_f32(0.1) {
            ev.send(AnimationChangeEvent {
                e: flower,
                new_anim: assets.anims["IDLE"].clone()
            });
        }

        if enemy.vel.length() <= 0.1
            && animator.anim.tex == assets.anims["MOVE"].tex
            && !patrol.lose_notice_timer.finished()
            && patrol.lost_target
        {
            ev.send(AnimationChangeEvent {
                e: flower,
                new_anim: assets.anims["IDLE"].clone()
            });
        }
    }
}

fn flower_enemy_move_on_patrol_resume(
    assets: Res<FlowerEnemyAssets>,
    q: Query<(&Enemy, Entity, &Patrol, &mut Animator), With<FlowerEnemy>>,
    mut ev: EventWriter<AnimationChangeEvent>
) {
    for (enemy, flower, patrol, animator) in q.iter() {
        let is_idling = animator.anim.tex == assets.anims["IDLE"].tex.clone();

        if patrol.patrol_pause_timer.finished() && enemy.vel.length() >= 0.1 && is_idling {
            ev.send(AnimationChangeEvent {
                e: flower,
                new_anim: assets.anims["MOVE"].clone()
            });
        }

        if patrol.lost_target == false && is_idling {
            ev.send(AnimationChangeEvent {
                e: flower,
                new_anim: assets.anims["MOVE"].clone()
            });
        }
    }
}

fn flower_enemy_move(
    assets: Res<FlowerEnemyAssets>,
    q: Query<Entity, (Added<Move>, With<FlowerEnemy>)>,
    mut ev: EventWriter<AnimationChangeEvent>
) {
    for flower in q.iter() {
        ev.send(AnimationChangeEvent {
            e: flower,
            new_anim: assets.anims["MOVE"].clone()
        });
    }
}

fn flower_enemy_detonate(
    assets: Res<FlowerEnemyAssets>,
    q: Query<Entity, (Added<Detonate>, With<FlowerEnemy>)>,
    mut ev: EventWriter<AnimationChangeEvent>
) {
    for flower in q.iter() {
        ev.send(AnimationChangeEvent {
            e: flower,
            new_anim: assets.anims["DETONATE"].clone()
        });
    }
}