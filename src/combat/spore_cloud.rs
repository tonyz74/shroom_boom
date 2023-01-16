use bevy::prelude::*;
use rand::prelude::*;
use bevy_rapier2d::prelude::*;
use seldom_state::prelude::{AlwaysTrigger, Done, DoneTrigger, NotTrigger, StateMachine};
use crate::assets::SporeAssets;
use crate::combat::{AttackStrength, CombatEvent, CombatLayerMask, KnockbackModifier};
use crate::combat::knockbacks::spore_cloud_knockback;
use crate::entity_states::*;
use crate::fx::spore::{SporeParticle, SporeParticleBundle};
use crate::state::GameState;

#[derive(Component, Debug, Clone)]
pub struct SporeCloudAttack {
    pub size: Vec2,
    pub dmg_timer: Timer,
    pub particle_timer: Timer,
    pub dur: Timer
}

impl Default for SporeCloudAttack {
    fn default() -> Self {
        Self {
            size: Vec2::new(32.0, 32.0),
            dmg_timer: Timer::from_seconds(0.8, TimerMode::Once),
            particle_timer: Timer::from_seconds(0.6, TimerMode::Repeating),
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
    pub knockback: KnockbackModifier,
    pub strength: AttackStrength,
    pub combat_layer: CombatLayerMask,
    pub state_machine: StateMachine
}

impl SporeCloudAttackBundle {
    pub fn new(pos: Vec2, size: Vec2) -> Self {
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
            knockback: KnockbackModifier::default(),
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
    assets: Res<SporeAssets>,
    mut q: Query<(Entity, &GlobalTransform, &mut SporeCloudAttack), Without<Die>>
) {
    for (entity, transform, mut spore_cloud) in q.iter_mut() {
        spore_cloud.dur.tick(time.delta());
        spore_cloud.dmg_timer.tick(time.delta());
        spore_cloud.particle_timer.tick(time.delta());

        if spore_cloud.dur.just_finished() {
            if let Some(mut cmd) = commands.get_entity(entity) {
                cmd.insert(Done::Success);
            }
        }

        if spore_cloud.particle_timer.just_finished() {
            let pos = transform.translation();
            let mut rng = thread_rng();

            let (x, y, rot, scale) = {
                let half_x = spore_cloud.size.x / 2.0;
                let half_y = spore_cloud.size.y / 2.0;

                let x = rng.gen_range((pos.x - half_x)..(pos.x + half_x));
                let y = rng.gen_range((pos.y - half_y)..(pos.y + half_y));

                let rot = rng.gen_range(-30.0..30.0);
                let scale = rng.gen_range(-0.5..0.0);

                (x, y, rot, scale)
            };

            commands.spawn(SporeParticleBundle {
                spore: SporeParticle {
                    rotation_speed: rot * (3.14 / 180.0),
                    scale_speed: scale,
                    ..default()
                },
                ..SporeParticleBundle::new(Vec2::new(x, y), &assets)
            });
        }
    }
}

fn spore_cloud_damage(
    mut clouds: Query<(
        &GlobalTransform,
        &Collider,
        &CombatLayerMask,
        &AttackStrength,
        &mut SporeCloudAttack,
        &KnockbackModifier
    ), Without<Die>>,
    rapier: Res<RapierContext>,
    combat_layers: Query<&CombatLayerMask>,
    mut hit_events: EventWriter<CombatEvent>,
) {
    for (transform, collider, combat_layer, atk, mut spore_cloud, kb) in clouds.iter_mut() {
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
                if !combat_layers.contains(hit_entity)
                    || combat_layers.get(hit_entity).unwrap().is_ally_with(*combat_layer) {
                    return true;
                }

                hit_events.send(CombatEvent {
                    target: hit_entity,
                    damage: atk.power,
                    kb: (kb.mod_fn)(spore_cloud_knockback())
                });

                true
            }
        );
    }
}
