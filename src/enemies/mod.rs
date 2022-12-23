use bevy::prelude::*;
use seldom_state::prelude::*;
use bevy_rapier2d::prelude::*;


use crate::common::AnimTimer;
use crate::pathfind::Pathfinder;

pub mod flower;

#[derive(Default, Component)]
pub struct Enemy {
    pub vel: Vec2
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub sensor: Sensor,
    pub anim_timer: AnimTimer,
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub state_machine: StateMachine,
    pub character_controller: KinematicCharacterController,
    pub path: Pathfinder,
    #[bundle]
    pub sprite_sheet: SpriteSheetBundle,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(flower::FlowerEnemyPlugin)
            .add_system(animate_enemies);
    }
}

pub fn animate_enemies(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>
    ), With<Enemy>>
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}
