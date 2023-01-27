
use bevy::prelude::*;
use std::time::Duration;

use crate::anim::{AnimationChangeEvent, Animator};
use crate::anim::map::AnimationMap;
use crate::enemies::Enemy;
use crate::state::GameState;
use crate::entity_states::*;
use crate::pathfind::{Pathfinder, Patrol};

pub fn register_enemy_animations(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(enemy_idle_on_patrol_pause)
            .with_system(enemy_move_on_patrol_resume)
            .with_system(enemy_idle)
            .with_system(enemy_move)
    );
}

fn enemy_idle_on_patrol_pause(
    q: Query<(&AnimationMap, &Enemy, Entity, &Patrol, &Animator), (With<Pathfinder>, Without<Die>)>,
    mut ev: EventWriter<AnimationChangeEvent>
) {
    for (anims, enemy, entity, patrol, animator) in q.iter() {

        if patrol.patrol_pause_timer.elapsed() <= Duration::from_secs_f32(0.1) {
            ev.send(AnimationChangeEvent {
                e: entity,
                new_anim: anims["IDLE"].clone()
            });
        }

        if enemy.vel.length() <= 0.1
            && animator.anim.name == "MOVE"
            && !patrol.lose_notice_timer.finished()
            && patrol.lost_target
        {
            ev.send(AnimationChangeEvent {
                e: entity,
                new_anim: anims["IDLE"].clone()
            });
        }
    }
}

fn enemy_move_on_patrol_resume(
    q: Query<(&AnimationMap, &Enemy, Entity, &Patrol, &mut Animator), (With<Pathfinder>, Without<Die>)>,
    mut ev: EventWriter<AnimationChangeEvent>
) {
    for (anims, enemy, entity, patrol, animator) in q.iter() {
        let is_idling = animator.anim.name == "IDLE";

        if patrol.patrol_pause_timer.finished() && enemy.vel.length() >= 0.1 && is_idling {
            ev.send(AnimationChangeEvent {
                e: entity,
                new_anim: anims["MOVE"].clone()
            });
        }

        if patrol.lost_target == false && is_idling {
            ev.send(AnimationChangeEvent {
                e: entity,
                new_anim: anims["MOVE"].clone()
            });
        }
    }
}

fn enemy_move(
    q: Query<(&AnimationMap, Entity), (Added<Move>, With<Pathfinder>, Without<Die>)>,
    mut ev: EventWriter<AnimationChangeEvent>
) {
    for (anims, entity) in q.iter() {
        ev.send(AnimationChangeEvent {
            e: entity,
            new_anim: anims["MOVE"].clone()
        });
    }
}

fn enemy_idle(
    q: Query<(&AnimationMap, Entity), (Added<Idle>, With<Pathfinder>, Without<Die>)>,
    mut ev: EventWriter<AnimationChangeEvent>
) {
    for (anims, entity) in q.iter() {
        ev.send(AnimationChangeEvent {
            e: entity,
            new_anim: anims["IDLE"].clone()
        });
    }
}
