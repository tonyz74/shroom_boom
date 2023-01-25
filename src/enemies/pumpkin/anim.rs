use bevy::prelude::*;
use crate::anim::{AnimationChangeEvent, Animator};
use crate::anim::map::AnimationMap;
use crate::enemies::Enemy;
use crate::enemies::pumpkin::PumpkinEnemy;
use crate::state::GameState;
use crate::entity_states::*;

pub fn register_pumpkin_enemy_animations(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(pumpkin_enemy_shoot)
            .with_system(pumpkin_enemy_idle_after_shoot)
            .with_system(pumpkin_enemy_move_after_shoot_wait)
    );
}

fn pumpkin_enemy_shoot(
    q: Query<(&AnimationMap, Entity), (Added<Shoot>, With<PumpkinEnemy>, Without<Die>)>,
    mut ev: EventWriter<AnimationChangeEvent>
) {
    for (anims, pumpkin) in q.iter() {
        ev.send(AnimationChangeEvent {
            e: pumpkin,
            new_anim: anims["SHOOT"].clone()
        });
    }
}

fn pumpkin_enemy_idle_after_shoot(
    texture_atlases: Res<Assets<TextureAtlas>>,
    q: Query<(&AnimationMap, &Animator, Entity), (With<PumpkinEnemy>, Without<Die>)>,
    mut ev: EventWriter<AnimationChangeEvent>
) {
    for (anims, animator, pumpkin) in q.iter() {
        if animator.anim.name != "SHOOT" {
            continue;
        }

        let frame_count = texture_atlases.get(&anims["SHOOT"].tex.clone()).unwrap().textures.len();
        if animator.total_frames >= frame_count as u32 - 1 {
            ev.send(AnimationChangeEvent {
                e: pumpkin,
                new_anim: anims["SHOOT_WAIT"].clone()
            });
        }
    }
}

fn pumpkin_enemy_move_after_shoot_wait(
    q: Query<(&Enemy, &AnimationMap, &Animator, Entity), (With<PumpkinEnemy>, Without<Die>)>,
    mut ev: EventWriter<AnimationChangeEvent>
) {
    for (enemy, anims, animator, pumpkin) in q.iter() {
        if enemy.vel.length() > 0.1 && animator.anim.name == "SHOOT_WAIT" {
            ev.send(AnimationChangeEvent {
                e: pumpkin,
                new_anim: anims["MOVE"].clone()
            });
        }
    }
}