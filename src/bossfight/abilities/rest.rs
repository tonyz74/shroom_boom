use bevy::prelude::*;
use bevy_rapier2d::prelude::Collider;
use seldom_state::prelude::*;
use crate::bossfight::{Boss, BossConfig};
use crate::bossfight::consts::{BOSS_HALF_SIZE, BOSS_FULL_SIZE, BOSS_HEAD_HALF_SIZE};
use crate::bossfight::enraged::EnragedAttackMove;
use crate::bossfight::stage::BossStage;
use crate::bossfight::state_machine::{AbilityStartup, Rest};
use crate::combat::{ColliderAttack, Immunity};
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
    mut colliders: Query<(&mut ColliderAttack, &mut Collider, &mut Transform)>,
    mut q: Query<(
        &Children,
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

    let (children, transform, mut immunity, mut rest, mut enemy, boss, cfg) = q.single_mut();
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
            for child in children {
                if let Ok((mut atk, mut collider, mut transform)) = colliders.get_mut(*child) {
                    *collider = Collider::cuboid(BOSS_HALF_SIZE.y / 2.0, BOSS_HALF_SIZE.x);
                    transform.translation.y = match next {
                        EnragedAttackMove::ChargeLeft => -1.0,
                        EnragedAttackMove::ChargeRight => 1.0,
                        _ => panic!()
                    } * BOSS_HALF_SIZE.y / 2.0;
                    atk.enabled = true;
                }
            }

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
