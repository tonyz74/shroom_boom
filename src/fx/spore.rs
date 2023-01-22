use bevy::prelude::*;
use bevy::time::FixedTimestep;
use crate::assets::SporeAssets;
use crate::common::{PHYSICS_STEP_DELTA, PHYSICS_STEPS_PER_SEC};
use crate::state::GameState;
use crate::anim::Animator;

#[derive(Component, Clone)]
pub struct SporeParticle {
    // Rotation speed in radians/sec
    pub rotation_speed: f32,
    pub scale_speed: f32,
    pub lifetime: Timer
}

impl Default for SporeParticle {
    fn default() -> Self {
        Self {
            rotation_speed: 0.0,
            scale_speed: 0.0,
            lifetime: Timer::from_seconds(2.0, TimerMode::Once)
        }
    }
}

#[derive(Bundle)]
pub struct SporeParticleBundle {
    #[bundle]
    pub sprite_sheet: SpriteSheetBundle,
    pub spore: SporeParticle,
    pub anim: Animator,
}

impl SporeParticleBundle {
    pub fn new(pos: Vec2, assets: &SporeAssets) -> Self {
        let anim = &assets.anims["SPORE"];

        Self {
            sprite_sheet: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::new(32.0, 32.0)),
                    ..default()
                },
                texture_atlas: anim.tex.clone(),
                transform: Transform::from_xyz(pos.x, pos.y, 10.0),
                ..default()
            },

            spore: SporeParticle {
                rotation_speed: 0.0,
                scale_speed: 0.0,
                lifetime: Timer::default()
            },

            anim: Animator::new(anim.clone())
        }
    }
}

pub fn register_spore_particles(app: &mut App) {
    app
        .add_system_set(
            SystemSet::on_update(GameState::Gameplay)
                .with_run_criteria(FixedTimestep::steps_per_second(PHYSICS_STEPS_PER_SEC))
                .with_system(spore_particle_rotate)
        )
        .add_system_set(
            SystemSet::on_update(GameState::Gameplay)
                .with_system(spore_particle_update)
        );
}

fn spore_particle_update(
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(Entity, &mut SporeParticle)>
) {
    for (entity, mut spore) in q.iter_mut() {
        spore.lifetime.tick(time.delta());

        if spore.lifetime.just_finished() {
            if let Some(cmd) = commands.get_entity(entity) {
                cmd.despawn_recursive();
            }
        }
    }
}

fn spore_particle_rotate(
    mut q: Query<(&mut Transform, &SporeParticle)>
) {
    for (mut transform, spore) in q.iter_mut() {
        transform.rotate_axis(Vec3::Z, spore.rotation_speed * PHYSICS_STEP_DELTA);

        let scale_diff = spore.scale_speed * PHYSICS_STEP_DELTA;
        *transform = transform.with_scale(transform.scale + scale_diff);
    }
}