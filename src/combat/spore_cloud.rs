use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use seldom_state::prelude::{AlwaysTrigger, Done, DoneTrigger, NotTrigger, StateMachine};
use crate::combat::{AttackStrength, CombatEvent, CombatLayerMask};
use crate::combat::knockbacks::spore_cloud_knockback;
use crate::entity_states::*;
use crate::state::GameState;


#[derive(Component, Debug, Clone)]
pub struct SporeCloudAttack {
    pub size: Vec2,
    pub dmg_timer: Timer,
    pub dur: Timer
}

impl Default for SporeCloudAttack {
    fn default() -> Self {
        Self {
            size: Vec2::new(32.0, 32.0),
            dmg_timer: Timer::from_seconds(0.8, TimerMode::Once),
            dur: Timer::from_seconds(8.0, TimerMode::Once)
        }
    }
}

fn spore_cloud_attack_state_machine() -> StateMachine {
    StateMachine::new(Active)
        .trans::<Active>(DoneTrigger::Success, Die { should_despawn: true })
        .trans::<Die>(NotTrigger(AlwaysTrigger), Active)
}

#[derive(Bundle)]
pub struct SporeCloudAttackBundle {
    pub sprite_sheet: SpriteBundle,
    pub collider: Collider,
    pub sensor: Sensor,
    pub attack: SporeCloudAttack,
    pub strength: AttackStrength,
    pub combat_layer: CombatLayerMask,
    pub state_machine: StateMachine
}

impl SporeCloudAttackBundle {
    pub fn from_pos(pos: Vec2, size: Vec2) -> Self {
        Self {
            sprite_sheet: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(size),
                    color: Color::rgba(0.0, 0.2, 1.0, 0.4),
                    ..default()
                },
                transform: Transform::from_xyz(pos.x, pos.y, 10.0),
                ..default()
            },

            collider: Collider::cuboid(size.x / 2., size.y / 2.),

            sensor: Sensor,

            attack: SporeCloudAttack {
                size,
                ..default()
            },

            strength: AttackStrength::new(2),

            combat_layer: CombatLayerMask::empty(),

            state_machine: spore_cloud_attack_state_machine()
        }
    }
}

pub fn register_spore_cloud_attacks(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(spore_cloud_update)
            .with_system(spore_cloud_damage)
    );
}

fn spore_cloud_update(
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(Entity, &mut SporeCloudAttack), Without<Die>>
) {
    for (entity, mut spore_cloud) in q.iter_mut() {
        spore_cloud.dur.tick(time.delta());
        spore_cloud.dmg_timer.tick(time.delta());

        if spore_cloud.dur.just_finished() {
            if let Some(mut cmd) = commands.get_entity(entity) {
                cmd.insert(Done::Success);
            }
        }
    }
}

fn spore_cloud_damage(
    mut spore_clouds: Query<(
        &GlobalTransform,
        &Collider,
        &CombatLayerMask,
        &AttackStrength,
        &mut SporeCloudAttack
    ), Without<Die>>,
    rapier: Res<RapierContext>,
    combat_layers: Query<&CombatLayerMask>,
    mut hit_events: EventWriter<CombatEvent>,
) {
    for (transform, collider, combat_layer, atk, mut spore_cloud) in spore_clouds.iter_mut() {
        if !spore_cloud.dmg_timer.finished() {
            continue;
        }

        spore_cloud.dmg_timer.reset();

        let spore_pos = transform.translation();

        rapier.intersections_with_shape(
            Vec2::new(spore_pos.x, spore_pos.y),
            Rot::default(),
            collider,
            QueryFilter {
                flags: QueryFilterFlags::ONLY_KINEMATIC,
                ..default()
            },
            |hit_entity| {
                if combat_layers.get(hit_entity).unwrap().is_ally_with(*combat_layer) {
                    return true;
                }

                hit_events.send(CombatEvent {
                    target: hit_entity,
                    damage: atk.power,
                    kb: spore_cloud_knockback()
                });

                true
            }
        );
    }
}
