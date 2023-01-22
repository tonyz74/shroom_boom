use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::combat::{AttackStrength, CombatLayerMask, KnockbackModifier};
use crate::combat::events::CombatEvent;
use crate::combat::knockbacks::melee_knockback;
use crate::anim::Animator;

#[derive(Bundle, Default)]
pub struct MeleeAttackBundle {
    pub anim: Animator,
    pub sprite_sheet: SpriteSheetBundle,
    pub collider: Collider,
    pub sensor: Sensor,
    pub attack: MeleeAttack,
    pub strength: AttackStrength,
    pub knockback: KnockbackModifier,
    pub combat_layer: CombatLayerMask
}

impl MeleeAttackBundle {
    pub fn from_size(size: Vec2) -> Self {
        Self {
            collider: Collider::cuboid(size.x / 2., size.y / 2.),
            sensor: Sensor,
            ..default()
        }
    }
}

#[derive(Component, Default, Debug, Copy, Clone)]
pub struct MeleeAttack {
    pub source: Option<Entity>
}

pub fn resolve_melee_attacks(
    transforms: Query<&GlobalTransform>,
    combat_layers_query: Query<&CombatLayerMask>,
    melees: Query<(
        &GlobalTransform,
        &Collider,
        &CombatLayerMask,
        &AttackStrength,
        &MeleeAttack,
        &KnockbackModifier
    )>,
    rapier: Res<RapierContext>,
    mut hit_events: EventWriter<CombatEvent>
) {
    for (transform, collider, combat_layer, atk, melee, kb) in melees.iter() {
        let atk_pos = if let Some(source) = melee.source {
            transforms.get(source).unwrap().translation()
        } else {
            transform.translation()
        };

        rapier.intersections_with_shape(
            Vect::new(transform.translation().x, transform.translation().y),
            Rot::default(),
            collider,
            QueryFilter {
                flags: QueryFilterFlags::ONLY_KINEMATIC,
                ..default()
            },
            |hit| {
                if let Ok(hit_combat_layer) = combat_layers_query.get(hit) {
                    // Skip if they're friendly (friendly fire is not enabled)
                    if hit_combat_layer.is_ally_with(*combat_layer) {
                        return true;
                    }

                    let hit_pos = transforms.get(hit).unwrap().translation();
                    let diff = (hit_pos - atk_pos).normalize();

                    // Only accept hits that have occurred between enemies
                    hit_events.send(CombatEvent {
                        target: hit,
                        damage: atk.power,
                        kb: (kb.mod_fn)(melee_knockback(Vec2::new(diff.x, diff.y)))
                    });
                }

                true
            }
        );
    }
}