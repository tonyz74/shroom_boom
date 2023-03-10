use bevy::prelude::*;
use crate::anim::{AnimationChangeEvent, Animator};
use crate::anim::map::AnimationMap;
use crate::bossfight::abilities::{RelocateAbility, RestAbility};
use crate::bossfight::Boss;
use crate::bossfight::enraged::EnragedAttackMove;
use crate::bossfight::stage::BossStage;

use crate::bossfight::state_machine::{BeginVulnerable, Boom, Hover, Relocate, Rest, Slam, Summon};
use crate::state::GameState;
use crate::entity_states::*;

pub fn register_boss_animations(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(boss_anim_boom)
            .with_system(boss_anim_retract)
            .with_system(boss_anim_extend)
            .with_system(boss_anim_slam)
            .with_system(boss_anim_early_flight)
            .with_system(boss_anim_hover)
            .with_system(boss_anim_rest)
            .with_system(boss_anim_summon)
            .with_system(boss_anim_begin_vulnerable)
            .with_system(boss_anim_vulnerable)
    );
}


fn boss_anim_summon(
    q: Query<(&AnimationMap, Entity), (Added<Summon>, With<Boss>, Without<Die>)>,
    mut ev: EventWriter<AnimationChangeEvent>
) {
    for (anims, boss) in q.iter() {
        ev.send(AnimationChangeEvent {
            e: boss,
            new_anim: anims["SUMMON"].clone()
        });
    }
}

fn boss_anim_begin_vulnerable(
    q: Query<(&AnimationMap, Entity), (Added<BeginVulnerable>, With<Boss>, Without<Die>)>,
    mut ev: EventWriter<AnimationChangeEvent>
) {
    for (anims, boss) in q.iter() {
        ev.send(AnimationChangeEvent {
            e: boss,
            new_anim: anims["VULNERABLE"].clone()
        });
    }
}

fn boss_anim_vulnerable(
    q: Query<(&Animator, &AnimationMap, Entity, &BossStage), (With<Boss>, Without<Die>)>,
    mut ev: EventWriter<AnimationChangeEvent>
) {
    for (animator, anims, boss, stage) in q.iter() {
        if ![
            BossStage::VulnerableEasy,
            BossStage::VulnerableMedium,
            BossStage::VulnerableHard
        ].contains(stage) {
            continue;
        }

        if animator.anim.name == anims["VULNERABLE"].name && animator.total_looped == 1 {
            ev.send(AnimationChangeEvent {
                e: boss,
                new_anim: anims["VULNERABLE_IDLE"].clone()
            });
        }
    }
}

fn boss_anim_hover(
    q: Query<(&AnimationMap, Entity), (Added<Hover>, With<Boss>, Without<Die>)>,
    mut ev: EventWriter<AnimationChangeEvent>
) {
    for (anims, boss) in q.iter() {
        ev.send(AnimationChangeEvent {
            e: boss,
            new_anim: anims["IDLE"].clone()
        });
    }
}

fn boss_anim_rest(
    q: Query<(&Boss, &AnimationMap, Entity), (Added<Rest>, With<Boss>, Without<Die>)>,
    mut ev: EventWriter<AnimationChangeEvent>
) {
    for (boss, anims, boss_e) in q.iter() {
        if boss.previous_move() != EnragedAttackMove::Slam {
            ev.send(AnimationChangeEvent {
                e: boss_e,
                new_anim: anims["IDLE"].clone()
            });
        }
    }
}

fn boss_anim_early_flight(
    q: Query<(&Boss, &AnimationMap, Entity, &RestAbility), (With<Rest>, With<Boss>, Without<Die>)>,
    mut ev: EventWriter<AnimationChangeEvent>
) {
    for (boss, anims, boss_e, rest) in q.iter() {
        let next = boss.next_move();

        if rest.timer.remaining_secs() <= 0.3 {
            let anim = match next {
                EnragedAttackMove::Takeoff | EnragedAttackMove::ChargeLeft | EnragedAttackMove::ChargeRight => {
                    anims["FLY"].clone()
                },
                EnragedAttackMove::Leap => {
                    anims["LEAP"].clone()
                },
                _ => continue
            };

            ev.send(AnimationChangeEvent {
                e: boss_e,
                new_anim: anim
            });
        }
    }
}

fn boss_anim_boom(
    q: Query<(&AnimationMap, Entity), (Added<Boom>, With<Boss>, Without<Die>)>,
    mut ev: EventWriter<AnimationChangeEvent>
) {
    for (anims, boss) in q.iter() {
        ev.send(AnimationChangeEvent {
            e: boss,
            new_anim: anims["BOOM"].clone()
        });
    }
}

fn boss_anim_retract(
    q: Query<(&AnimationMap, Entity), (Added<Relocate>, With<Boss>, Without<Die>)>,
    mut ev: EventWriter<AnimationChangeEvent>
) {
    for (anims, boss) in q.iter() {
        ev.send(AnimationChangeEvent {
            e: boss,
            new_anim: anims["RETRACT"].clone()
        });
    }
}

fn boss_anim_extend(
    q: Query<(&RelocateAbility, &AnimationMap, Entity), (With<Relocate>, With<Boss>, Without<Die>)>,
    mut ev: EventWriter<AnimationChangeEvent>
) {
    for (relocate, anims, boss) in q.iter() {
        if relocate.retract.finished() {
            ev.send(AnimationChangeEvent {
                e: boss,
                new_anim: anims["EXTEND"].clone()
            });
        }
    }
}

fn boss_anim_slam(
    q: Query<(&AnimationMap, Entity), (Added<Slam>, With<Boss>, Without<Die>)>,
    mut ev: EventWriter<AnimationChangeEvent>
) {
    for (anims, boss) in q.iter() {
        ev.send(AnimationChangeEvent {
            e: boss,
            new_anim: anims["SLAM"].clone()
        });
    }
}