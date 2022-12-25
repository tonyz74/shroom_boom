use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::attack::{AttackStrength, CombatLayerMask};
use crate::attack::events::HitEvent;
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
pub struct MeleeAttack;

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
        &AttackStrength
    ), With<MeleeAttack>>,
    rapier: Res<RapierContext>,
    mut hit_events: EventWriter<HitEvent>
) {
    for (transform, collider, combat_layer, atk) in melees.iter() {
        let atk_pos = transform.translation();

        rapier.intersections_with_shape(
            Vect::new(atk_pos.x, atk_pos.y),
            Rot::default(),
            collider,
            QueryFilter {
                flags: QueryFilterFlags::ONLY_KINEMATIC,
                ..default()
            },
            |hit| {
                if let Ok(hit_combat_layer) = combat_layers_query.get(hit) {
                    if hit_combat_layer.is_ally_with(*combat_layer) {
                        return true;
                    }

                    let hit_pos = transforms.get(hit).unwrap().translation();
                    let dir = Vec2::new(hit_pos.x - atk_pos.x, 0.0).normalize().x;

                    // Only accept hits that have occurred between enemies
                    hit_events.send(HitEvent {
                        target: hit,
                        damage: atk.power,
                        kb: Vec2::new(dir * 5.0, 3.0)
                    });
                }

                true
            }
        );
    }
}