use bevy::prelude::*;
use seldom_state::prelude::*;
use crate::coin::drops::CoinHolder;
use crate::combat::{ColliderAttack, CombatLayerMask, ExplosionEvent, Immunity};
use crate::enemies::Enemy;
use crate::enemies::flower::FlowerEnemy;
use crate::entity_states::*;
use crate::fx::indicator::Indicator;
use crate::pathfind::{Pathfinder, Region};
pub use crate::pathfind::state_machine::*;
use crate::state::GameState;


const BOOM_SIZE: f32 = 72.0;

#[derive(Copy, Clone, Debug, Reflect, Component)]
pub struct Detonate;

#[derive(Copy, Clone, Debug, Reflect, FromReflect, Component)]
pub struct DetonateTrigger;

impl Trigger for DetonateTrigger {
    type Param<'w, 's> = Query<'w, 's,
        (&'static Pathfinder, &'static GlobalTransform),
        With<FlowerEnemy>
    >;

    fn trigger(&self, entity: Entity, q: &Self::Param<'_, '_>) -> bool {
        if !q.contains(entity) {
            return false;
        }

        let (pathfinder, tf) = q.get(entity).unwrap();
        let pos = tf.translation();

        if let Some(target) = pathfinder.target {
            let dist = target.distance(Vec2::new(pos.x, pos.y));
            return dist <= 48.0;
        }

        false
    }
}

pub fn register_flower_enemy_state_machine(app: &mut App) {
    app
        .add_plugin(TriggerPlugin::<DetonateTrigger>::default())
        .add_system_set(
            SystemSet::on_update(GameState::Gameplay)
                .with_system(flower_enemy_detonate)
                .with_system(flower_enemy_tick)
                .with_system(flower_enemy_disable_collider_on_detonate)
        );
}

pub fn flower_enemy_disable_collider_on_detonate(
    mut p: Query<&mut ColliderAttack>,
    q: Query<&Children, With<Detonate>>
) {
    for children in q.iter() {
        for child in children {
            if let Ok(mut atk) = p.get_mut(*child) {
                atk.enabled = false;
            }
        }
    }
}

pub fn flower_enemy_detonate(
    mut q: Query<(
        &GlobalTransform,
        &mut Pathfinder,
        &mut Enemy,
        &mut FlowerEnemy,
        &mut Immunity
    ), Added<Detonate>>,
    mut indicators: EventWriter<Indicator>
) {
    for (transform, mut pathfinder, mut enemy, mut flower, mut immunity) in q.iter_mut() {
        let pos = transform.translation();

        pathfinder.active = false;
        enemy.vel = Vec2::ZERO;
        immunity.is_immune = true;
        flower.countdown.reset();

        indicators.send(
            Indicator {
                region: Region {
                    tl: Vec2::new(pos.x, pos.y) + Vec2::new(-BOOM_SIZE, BOOM_SIZE),
                    br: Vec2::new(pos.x, pos.y) + Vec2::new(BOOM_SIZE, -BOOM_SIZE)
                },

                wait_time: 0.0,
                expand_time: 0.2,

                ..Indicator::EXPLOSION
            }
        );
    }
}

pub fn flower_enemy_tick(
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(
        Entity,
        &GlobalTransform,
        &mut FlowerEnemy,
        &mut CoinHolder
    ), With<Detonate>>,

    mut explosions: EventWriter<ExplosionEvent>
) {
    for (entity, tf, mut flower, mut coin_holder) in q.iter_mut() {
        let pos = tf.translation();
        flower.countdown.tick(time.delta());

        if flower.countdown.just_finished() {
            commands.entity(entity).insert(Done::Success);
            coin_holder.total_value = 0;

            explosions.send(
                ExplosionEvent {
                    pos: Vec2::new(pos.x, pos.y),
                    max_damage: flower.explosion_power,
                    radius: BOOM_SIZE,
                    combat_layer: CombatLayerMask::empty()
                }
            );
        }
    }
}


pub fn flower_enemy_state_machine() -> StateMachine {
    walk_pathfinder_state_machine()
        .trans::<Move>(DetonateTrigger, Detonate)
        .trans::<Detonate>(DoneTrigger::Success, Die::default())
}