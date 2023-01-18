use bevy::prelude::*;
use seldom_state::prelude::*;
use crate::coin::drops::CoinHolder;
use crate::combat::{ColliderAttack, ExplosionEvent, Immunity};
use crate::enemies::Enemy;
use crate::enemies::flower::FlowerEnemy;
use crate::entity_states::*;
use crate::fx::indicator::Indicator;
use crate::pathfind::{Pathfinder, Region};
pub use crate::pathfind::state_machine::*;
use crate::state::GameState;

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
        );
}

pub fn flower_enemy_detonate(
    mut p: Query<&mut ColliderAttack>,
    mut q: Query<(
        &Children,
        &GlobalTransform,
        &mut Pathfinder,
        &mut Enemy,
        &mut FlowerEnemy,
        &mut Immunity
    ), Added<Detonate>>,
    mut indicators: EventWriter<Indicator>
) {
    for (kids, transform, mut pathfinder, mut enemy, mut flower, mut immunity) in q.iter_mut() {

        for child in kids {
            if let Ok(mut atk) = p.get_mut(*child) {
                atk.enabled = false;
            }
        }

        let pos = transform.translation();

        pathfinder.active = false;
        enemy.vel = Vec2::ZERO;
        immunity.is_immune = true;
        flower.countdown.reset();

        indicators.send(
            Indicator {
                region: Region {
                    tl: Vec2::new(pos.x, pos.y) + Vec2::new(-40.0, 40.0),
                    br: Vec2::new(pos.x, pos.y) + Vec2::new(40.0, -40.0)
                },

                wait_time: 0.2,
                expand_time: 0.3,

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
                    radius: 40.0
                }
            );
        }
    }
}


pub fn flower_enemy_state_machine() -> StateMachine {
    walk_pathfinder_state_machine()
        .trans::<Move>(DetonateTrigger, Detonate)
        .trans::<Fall>(DetonateTrigger, Detonate)
        .trans::<Detonate>(DoneTrigger::Success, Die::default())
}