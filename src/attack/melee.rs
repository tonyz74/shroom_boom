use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::common::{Anim, AnimTimer};

#[derive(Component, Debug, Copy, Clone)]
pub struct MeleeAttack {
    pub source: Entity,
    pub damage: i32,
}

impl MeleeAttack {
    pub fn spawn(
        mut commands: Commands,
        attack: MeleeAttack,
        pos: Vec2,
        size: Vec2,
        anim: Anim
    ) -> Entity {
        commands.spawn((

            SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(size),
                    ..default()
                },

                texture_atlas: anim.tex.clone(),
                transform: Transform::from_xyz(pos.x + 16., pos.y, 0.5),
                ..default()
            },

            AnimTimer::from_seconds(anim.speed),

            Collider::cuboid(size.x / 2., size.y / 2.),
            Sensor,

            attack

        )).id()
    }
}

pub fn animate_melee(
    mut commands: Commands,
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        Entity,
        &mut AnimTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>
    ), With<MeleeAttack>>
) {
    for (ent, mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();

            if sprite.index == 0 {
                commands.entity(ent).despawn();
            }
        }
    }
}
