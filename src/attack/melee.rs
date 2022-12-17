use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::common::AnimTimer;

#[derive(Bundle, Default)]
pub struct MeleeAttackBundle {
    pub anim_timer: AnimTimer,
    pub sprite_sheet: SpriteSheetBundle,
    pub collider: Collider,
    pub sensor: Sensor,
    pub attack: MeleeAttack
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
    pub source: Option<Entity>,
    pub damage: i32,
    pub offset: Vec2,
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