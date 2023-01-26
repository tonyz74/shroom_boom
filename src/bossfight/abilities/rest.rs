use bevy::prelude::*;
use seldom_state::prelude::*;
use crate::bossfight::{Boss, BossConfig};
use crate::bossfight::consts::{BOSS_HALF_SIZE, BOSS_FULL_SIZE, BOSS_HEAD_HALF_SIZE};
use crate::bossfight::enraged::EnragedAttackMove;
use crate::bossfight::stage::BossStage;
use crate::bossfight::state_machine::{AbilityStartup, Rest};
use crate::combat::Immunity;
use crate::enemies::Enemy;
use crate::fx::indicator::Indicator;
use crate::pathfind::Region;
use crate::state::GameState;

#[derive(Debug, Component, Clone)]
pub struct RestAbility {
    pub timer: Timer
}


impl Default for RestAbility {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.0, TimerMode::Once)
        }
    }
}


pub fn register_rest_ability(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(start_resting)
            .with_system(rest_update)
    );
}


fn start_resting(
    mut q: Query<(
        &GlobalTransform,
        &mut Immunity,
        &mut RestAbility,
        &mut Enemy,
        &Boss,
        &BossConfig
    ), Added<AbilityStartup>>,

    mut indicators: EventWriter<Indicator>
) {
    if q.is_empty() {
        return;
    }

    let (transform, mut immunity, mut rest, mut enemy, boss, cfg) = q.single_mut();
    let pos = transform.translation();

    let len = match boss.current_move() {
        EnragedAttackMove::Rest(n) => n,
        _ => return
    };

    enemy.vel = Vec2::ZERO;

    rest.timer.reset();
    rest.timer.set_duration(std::time::Duration::from_secs_f32(len));

    immunity.is_immune = false;


    // Preemptively draw indicators
    let next = boss.next_move();
    match next {
        EnragedAttackMove::ChargeLeft | EnragedAttackMove::ChargeRight => {
            let region = match next {
                EnragedAttackMove::ChargeLeft => Region {
                    tl: Vec2::new(
                        cfg.charge_left.x - BOSS_HALF_SIZE.y,
                        cfg.charge_left.y + BOSS_HEAD_HALF_SIZE.y
                    ),
                    br: Vec2::new(
                        cfg.charge_left.x + BOSS_FULL_SIZE.x,
                        cfg.charge_left.y - BOSS_HEAD_HALF_SIZE.y
                    ),
                },
                EnragedAttackMove::ChargeRight => Region {
                    tl: Vec2::new(
                        cfg.charge_right.x - BOSS_FULL_SIZE.x,
                        cfg.charge_right.y + BOSS_HEAD_HALF_SIZE.y
                    ),
                    br: Vec2::new(
                        cfg.charge_right.x + BOSS_FULL_SIZE.x,
                        cfg.charge_right.y - BOSS_HEAD_HALF_SIZE.y
                    ),
                },
                _ => panic!("")
            };

            indicators.send(Indicator {
                region,
                wait_time: 1.2,
                expand_time: 0.5,
                ..Indicator::ATTACK
            });
        },

        EnragedAttackMove::Slam => {
            indicators.send(
                Indicator {
                    region: Region {
                        tl: Vec2::new(
                            pos.x - BOSS_HALF_SIZE.x,
                            cfg.slam_base.y - BOSS_HALF_SIZE.y + 64.0
                        ),

                        br: Vec2::new(
                            pos.x + BOSS_HALF_SIZE.x,
                            cfg.slam_base.y - BOSS_HALF_SIZE.y
                        )
                    },
                    wait_time: 0.4,
                    expand_time: 0.2,
                    ..Indicator::ATTACK
                }
            );
        },

        _ => {}
    }
}

fn rest_update(
    time: Res<Time>,
    mut commands: Commands,
    resting: Query<&Rest>,
    mut q: Query<(Entity, &mut RestAbility, &BossStage, &Boss)>
) {
    if q.is_empty() {
        return;
    }

    let (entity, mut rest, stage, boss) = q.single_mut();

    let is_resting = match boss.current_move() {
        EnragedAttackMove::Rest(_) => true,
        _ => false
    };

    if stage != &BossStage::Enraged || !is_resting {
        return;
    }

    rest.timer.tick(time.delta());
    if rest.timer.finished() && resting.contains(entity) {
        commands.entity(entity).insert(Done::Success);
    }
}
