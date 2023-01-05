use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::combat::{AttackStrength, CombatLayerMask};
use crate::combat::events::HitEvent;
use crate::common::AnimTimer;

#[derive(Bundle, Default)]
pub struct MeleeAttackBundle {
    pub anim_timer: AnimTimer,
    pub sprite_sheet: SpriteSheetBundle,
    pub collider: Collider,
    pub sensor: Sensor,
    pub attack: MeleeAttack,
    pub strength: AttackStrength,
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

pub fn animate_melee(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>
    ), With<MeleeAttack>>
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}


pub fn resolve_melee_attacks(
    transforms: Query<&GlobalTransform>,
    combat_layers_query: Query<&CombatLayerMask>,
    melees: Query<(
        &GlobalTransform,
        &Collider,
        &CombatLayerMask,
        &AttackStrength,
        &MeleeAttack
    )>,
    rapier: Res<RapierContext>,
    mut hit_events: EventWriter<HitEvent>
) {
    for (transform, collider, combat_layer, atk, melee) in melees.iter() {
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
                    hit_events.send(HitEvent {
                        target: hit,
                        damage: atk.power,
                        kb: Vec2::new(diff.x, diff.y)
                    });
                }

                true
            }
        );
    }
}