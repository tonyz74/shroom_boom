use bevy::prelude::*;
use rand::prelude::*;
use seldom_state::prelude::*;
use crate::bossfight::{Boss, BossConfig, util};
use crate::bossfight::consts::{BOSS_BOOM_EXPLOSION_COUNT, BOSS_BOOM_PARTITION_SIZE, BOSS_BOOM_SELECTION_TIME, BOSS_BOOM_SUMMON_WAIT_TIME, BOSS_BOOM_WAIT_TIME};
use crate::bossfight::enraged::EnragedAttackMove;
use crate::bossfight::stage::BossStage;
use crate::bossfight::state_machine::{AbilityStartup, Boom};
use crate::bossfight::summon::SummonAbility;
use crate::bossfight::util::pick_point_in_region;
use crate::combat::{CombatLayerMask, ExplosionEvent, Immunity};
use crate::enemies::spawner::{EnemyDifficulty, EnemyLocation, EnemySpawnEvent, EnemyType};
use crate::fx::indicator::Indicator;
use crate::pathfind::Region;
use crate::state::GameState;

#[derive(Component, Debug, Clone)]
pub struct BoomAbility {
    pub sel_timer: Timer,
    pub explosion_points: Vec<Vec2>,
    pub enemy_points: Vec<(EnemyType, Vec2)>,
    pub wait_timer: Timer,
    pub summon_wait_timer: Timer,
}

impl Default for BoomAbility {
    fn default() -> Self {
        Self {
            sel_timer: Timer::from_seconds(BOSS_BOOM_SELECTION_TIME, TimerMode::Repeating),
            explosion_points: vec![],
            enemy_points: vec![],
            wait_timer: Timer::from_seconds(BOSS_BOOM_WAIT_TIME, TimerMode::Once),
            summon_wait_timer: Timer::from_seconds(BOSS_BOOM_SUMMON_WAIT_TIME, TimerMode::Once)
        }
    }
}

pub fn register_boom_ability(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(start_booming)
            .with_system(boom_update)
    );
}

fn start_booming(
    mut q: Query<(
        &mut Immunity,
        &mut BoomAbility,
        &Boss,
        &BossConfig
    ), Added<AbilityStartup>>,
    mut indicators: EventWriter<Indicator>
) {
    if q.is_empty() {
        return;
    }

    let (mut immunity, mut boom, boss, cfg) = q.single_mut();
    if boss.current_move() != EnragedAttackMove::Boom {
        return;
    }

    immunity.is_immune = true;

    boom.wait_timer.reset();
    boom.summon_wait_timer.reset();
    boom.sel_timer.reset();
    boom.explosion_points.clear();
    boom.enemy_points.clear();

    // Summon 2 enemies & a dandelion
    let ty_list = [EnemyType::Tumbleweed, EnemyType::Pumpkin, EnemyType::Flower];
    let mut rng = thread_rng();

    let chosen = [
        ty_list[rng.gen_range(0..(ty_list.len() - 1))],
        EnemyType::Dandelion
    ];

    for i in chosen {
        let mut p;
        loop {
            p = pick_point_in_region(cfg.summon_region, BOSS_BOOM_PARTITION_SIZE);
            if boom.enemy_points.iter().find(|i| i.1 == p).is_none() {
                break;
            }
        }

        boom.enemy_points.push((i, p));
        indicators.send(Indicator {
            region: Region {
                tl: p + Vec2::new(-24.0, 24.0),
                br: p + Vec2::new(24.0, -24.0)
            },
            wait_time: BOSS_BOOM_SUMMON_WAIT_TIME + BOSS_BOOM_SELECTION_TIME * BOSS_BOOM_EXPLOSION_COUNT as f32 - 0.4,
            expand_time: 0.4,
            ..Indicator::SPAWNER
        });
    }
}


fn boom_update(
    time: Res<Time>,
    mut commands: Commands,
    booming: Query<&Boom>,

    mut q: Query<(
        Entity,
        &mut BoomAbility,
        &mut Immunity,
        &Boss,
        &BossStage,
        &BossConfig,
        &SummonAbility
    )>,

    mut events: EventWriter<ExplosionEvent>,
    mut spawn_events: EventWriter<EnemySpawnEvent>,
    mut indicators: EventWriter<Indicator>
) {
    if q.is_empty() {
        return;
    }

    let (entity, mut boom, mut immunity, boss, stage, cfg, summon) = q.single_mut();
    if stage != &BossStage::Enraged || boss.current_move() != EnragedAttackMove::Boom {
        return;
    }

    if boom.explosion_points.len() < BOSS_BOOM_EXPLOSION_COUNT {
        boom.sel_timer.tick(time.delta());

        if boom.sel_timer.just_finished() {
            let mut point;

            loop {
                point = pick_point_in_region(cfg.boom_region, BOSS_BOOM_PARTITION_SIZE);

                if !boom.explosion_points.contains(&point) {
                    break;
                }
            }

            let len = boom.explosion_points.len();
            let wait = (
                (BOSS_BOOM_SELECTION_TIME * (BOSS_BOOM_EXPLOSION_COUNT - len) as f32)
                    + BOSS_BOOM_WAIT_TIME - 0.5
            ).clamp(0.0, f32::MAX);

            indicators.send(
                Indicator {
                    region: Region {
                        tl: point + Vec2::new(-60.0, 60.0),
                        br: point + Vec2::new(60.0, -60.0),
                    },
                    wait_time: wait,
                    expand_time: 0.4,
                    ..Indicator::EXPLOSION
                },
            );

            boom.explosion_points.push(point);
        }

    } else {
        boom.wait_timer.tick(time.delta());
        boom.summon_wait_timer.tick(time.delta());

        if boom.summon_wait_timer.just_finished() {
            boom_spawn_enemies(&boom.enemy_points, &mut spawn_events, &summon.total_region);
        }

        if boom.wait_timer.finished() && booming.contains(entity) {
            boom_spawn_explosions(&boom.explosion_points, &mut events);
            immunity.is_immune = true;

            commands.entity(entity).insert(Done::Success);
        }
    }
}

fn boom_spawn_explosions(
    points: &[Vec2],
    events: &mut EventWriter<ExplosionEvent>
) {
    for point in points {
        events.send(ExplosionEvent {
            pos: *point,
            max_damage: 80,
            radius: 60.0,
            combat_layer: CombatLayerMask::ENEMY
        });
    }
}

fn boom_spawn_enemies(
    points: &[(EnemyType, Vec2)],
    events: &mut EventWriter<EnemySpawnEvent>,
    reg: &Region
) {
    for (ty, point) in points {
        println!("location: {:?}", point);
        events.send(EnemySpawnEvent {
            ty: *ty,
            coins: 0,
            difficulty: EnemyDifficulty::Hard,
            location: EnemyLocation {
                pos: *point,
                patrol_region: *reg
            },
            rand_range: 0.9..1.1,
            extra_components: None
        });
    }
}