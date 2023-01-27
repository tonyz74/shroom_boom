use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use seldom_state::prelude::*;
use crate::anim::Animator;
use crate::assets::ExplosionAssets;
use crate::combat::{AttackStrength, CombatEvent, CombatLayerMask, KnockbackModifier};
use crate::combat::consts::{EXPLOSION_DIAMETER, EXPLOSION_DURATION, EXPLOSION_EFFECTIVE_DURATION, EXPLOSION_RADIUS};
use crate::combat::knockbacks::explosion_knockback;
use crate::entity_states::*;
use crate::fx::shake::ScreenShakeEvent;
use crate::state::GameState;

#[derive(Component, Clone)]
pub struct ExplosionAttack {
    pub dur: Timer,
    pub effective_dur: Timer,
}

impl Default for ExplosionAttack {
    fn default() -> Self {
        Self {
            dur: Timer::from_seconds(EXPLOSION_DURATION, TimerMode::Once),
            effective_dur: Timer::from_seconds(EXPLOSION_EFFECTIVE_DURATION, TimerMode::Once)
        }
    }
}




fn explosion_attack_state_machine() -> StateMachine {
    StateMachine::new(Active)
        .trans::<Active>(DoneTrigger::Success, Die::default())
        .trans::<Die>(NotTrigger(AlwaysTrigger), Active)
}


#[derive(Bundle)]
pub struct ExplosionAttackBundle {
    pub anim: Animator,
    pub sprite_sheet: SpriteSheetBundle,
    pub collider: Collider,
    pub sensor: Sensor,
    pub attack: ExplosionAttack,
    pub knockback: KnockbackModifier,
    pub strength: AttackStrength,
    pub combat_layer: CombatLayerMask,
    pub state_machine: StateMachine
}

impl ExplosionAttackBundle {
    pub fn new(pos: Vec2, assets: &ExplosionAssets) -> Self {
        Self {
            anim: Animator::new(assets.anims["BOOM"].clone()),

            sprite_sheet: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::splat(EXPLOSION_DIAMETER)),
                    ..default()
                },
                texture_atlas: assets.anims["BOOM"].tex.clone(),
                transform: Transform::from_xyz(pos.x, pos.y, 10.0),
                ..default()
            },

            collider: Collider::ball(0.0),
            sensor: Sensor,
            attack: ExplosionAttack::default(),
            strength: AttackStrength::default(),
            knockback: KnockbackModifier::default(),
            combat_layer: CombatLayerMask::ENEMY,
            state_machine: explosion_attack_state_machine()
        }
    }
}

pub fn register_explosion_attacks(app: &mut App) {
    app
        .add_event::<ExplosionEvent>()
        .add_system_set(
            SystemSet::on_update(GameState::Gameplay)
                .with_system(tick_explosion_timers)
                .with_system(explosion_expand)
                .with_system(explosion_death)
                .with_system(explosion_damage)
                .with_system(explosion_events)
        );
}


fn tick_explosion_timers(
    time: Res<Time>,
    mut q: Query<(Entity, &mut ExplosionAttack)>,
    mut commands: Commands
) {
    for (entity, mut explosion) in q.iter_mut() {
        explosion.dur.tick(time.delta());
        explosion.effective_dur.tick(time.delta());

        if explosion.effective_dur.just_finished() {
            if let Some(mut cmd) = commands.get_entity(entity) {
                cmd.insert(Done::Success);
            }
        }

    }
}

fn explosion_death(mut q: Query<(&ExplosionAttack, &mut Die)>) {
    for (explosion, mut death) in q.iter_mut() {
        if explosion.dur.finished() {
            death.should_despawn = true;
        }
    }
}

fn explosion_expand(mut q: Query<(&mut Collider, &ExplosionAttack)>) {
    for (mut collider, explosion) in q.iter_mut() {
        *collider = Collider::ball(explosion.effective_dur.percent() * EXPLOSION_RADIUS);
    }
}

fn explosion_damage(
    transforms: Query<&GlobalTransform>,
    mut explosions: Query<(
        Entity,
        &GlobalTransform,
        &Collider,
        &CombatLayerMask,
        &AttackStrength,
        &KnockbackModifier
    ), (With<ExplosionAttack>, Without<Die>)>,
    rapier: Res<RapierContext>,

    combat_layers: Query<&CombatLayerMask>,
    mut hit_events: EventWriter<CombatEvent>,
) {
    for (entity, tf, collider, combat_layer, atk, kb) in explosions.iter_mut() {
        let scale = {
            let (s, _r, _t) = tf.to_scale_rotation_translation();
            (s.x + s.y) / 2.0
        };

        let transform = transforms.get(entity).unwrap();
        let explosion_pos = transform.translation();

        rapier.intersections_with_shape(
            Vec2::new(explosion_pos.x, explosion_pos.y),
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

                let hit_pos = transforms.get(hit_entity).unwrap().translation();
                let diff = (hit_pos - explosion_pos).xy();
                let percentage = 1.0 - (diff.length() / (EXPLOSION_DIAMETER * scale));

                hit_events.send(CombatEvent {
                    target: hit_entity,
                    damage: (atk.power as f32 * percentage).abs().clamp(0.0, atk.power as f32) as i32,
                    kb: (kb.mod_fn)(explosion_knockback(diff, EXPLOSION_DIAMETER * scale))
                });

                true
            }
        );
    }
}

#[derive(Component, Copy, Clone, Resource)]
pub struct ExplosionEvent {
    pub pos: Vec2,
    pub max_damage: i32,
    pub radius: f32,
    pub combat_layer: CombatLayerMask
}

fn explosion_events(
    mut commands: Commands,
    assets: Res<ExplosionAssets>,
    mut explosions: EventReader<ExplosionEvent>,
    mut shakes: EventWriter<ScreenShakeEvent>
) {
    for explosion in explosions.iter() {
        let mut atk = ExplosionAttackBundle::new(explosion.pos, &assets);
        atk.sprite_sheet.transform.scale = Vec2::splat(explosion.radius / EXPLOSION_RADIUS).extend(1.0);
        atk.strength.power = explosion.max_damage;
        atk.combat_layer = explosion.combat_layer;

        shakes.send(ScreenShakeEvent::TINY);

        commands.spawn(atk);
    }
}